mod contracts;

use ethcontract::prelude::*;
use ethcontract::web3::Transport;
use intmax_config::EthConfig;
use secp256k1::SecretKey;
use std::str::FromStr;

pub struct EthProvider<T: Transport> {
    web3: Web3<T>,
    secret_key: SecretKey,
}

impl<T: Transport> EthProvider<T> {
    fn new(transport: T, config: &EthConfig) -> Self {
        let secret_key =
            SecretKey::from_str(config.committer_key.as_str()).expect("failed to load secret_key");

        EthProvider {
            web3: Web3::new(transport),
            secret_key,
        }
    }
}

#[cfg(test)]
mod tests {

    use ethcontract::futures::io::{BufReader, BufWriter};
    use ethcontract::{futures, Address, Http, H160};

    use crate::EthProvider;
    use intmax_config::{EthConfig, Scheme};
    use secp256k1::{PublicKey, Secp256k1};

    use ethcontract::web3::signing::keccak256;
    use ethcontract::web3::transports::ws::compat;
    use ethcontract::web3::transports::WebSocket;
    use soketto::handshake;
    use tokio_stream::wrappers::TcpListenerStream;
    use tokio_stream::StreamExt;

    trait EthAddress {
        fn to_eth_address(&self) -> H160;
    }

    impl EthAddress for PublicKey {
        // refs: https://github.com/tomusdrw/rust-web3/blob/8ef0ca3cb8e6783a4cb43d6f499354b6466e2394/src/signing.rs#L158-L165
        fn to_eth_address(&self) -> H160 {
            let pub_key = self.serialize_uncompressed();

            let hash = keccak256(&pub_key[1..]);
            Address::from_slice(&hash[12..])
        }
    }

    async fn server(listener: compat::TcpListener) {
        let mut incoming = TcpListenerStream::new(listener);
        while let Some(Ok(socket)) = incoming.next().await {
            let socket = compat::compat(socket);
            let mut server = handshake::Server::new(BufReader::new(BufWriter::new(socket)));
            let key = {
                let req = server.receive_request().await.unwrap();
                req.key()
            };
            let accept = handshake::server::Response::Accept {
                key,
                protocol: None,
            };
            server.send_response(&accept).await.unwrap();
            let (mut sender, mut receiver) = server.into_builder().finish();
            loop {
                let mut data = Vec::new();
                match receiver.receive_data(&mut data).await {
                    Ok(data_type) if data_type.is_text() => {
                        assert_eq!(
                            std::str::from_utf8(&data),
                            Ok(r#"{"jsonrpc":"2.0","method":"eth_accounts","params":[],"id":1}"#)
                        );
                        sender
                            .send_text(r#"{"jsonrpc":"2.0","id":1,"result":"x"}"#)
                            .await
                            .unwrap();
                        sender.flush().await.unwrap();
                    }
                    Err(soketto::connection::Error::Closed) => break,
                    e => panic!("Unexpected data: {:?}", e),
                }
            }
        }
    }

    #[tokio::test]
    async fn success_eth_provider_new_with_http() {
        let config = EthConfig {
            committer_key: "10d18ee85b1a2e1d4b47feed91074a6bb4a17b55005144338208a0be031752d3"
                .to_string(),
            port: 8545,
            host: "127.0.0.1".to_string(),
            scheme: Scheme::Http,
        };

        let transport = Http::new(&config.node_url()).unwrap();
        let provider = EthProvider::new(transport, &config);
        assert_eq!(
            provider.secret_key.to_string(),
            "10d18ee85b1a2e1d4b47feed91074a6bb4a17b55005144338208a0be031752d3"
        );

        // test some api
        let sig = provider.web3.accounts().sign("hoge", &provider.secret_key);
        let recovered_address = provider.web3.accounts().recover(&sig).unwrap();

        let secp = Secp256k1::new();
        let expected_address =
            PublicKey::from_secret_key(&secp, &provider.secret_key).to_eth_address();

        assert_eq!(recovered_address, expected_address);
    }

    #[tokio::test]
    async fn success_eth_provider_new_with_ws() {
        let config = EthConfig {
            committer_key: "10d18ee85b1a2e1d4b47feed91074a6bb4a17b55005144338208a0be031752d3"
                .to_string(),
            port: 8544,
            host: "127.0.0.1".to_string(),
            scheme: Scheme::Ws,
        };

        let addr = config.node_url();
        let listener = futures::executor::block_on(compat::TcpListener::bind("127.0.0.1:8544"))
            .expect("Failed to bind");
        tokio::spawn(server(listener));

        let transport = WebSocket::new(addr.as_str()).await.unwrap();
        let provider = EthProvider::new(transport, &config);
        assert_eq!(
            provider.secret_key.to_string(),
            "10d18ee85b1a2e1d4b47feed91074a6bb4a17b55005144338208a0be031752d3"
        );

        // test some api
        let sig = provider.web3.accounts().sign("fuga", &provider.secret_key);
        let recovered_address = provider.web3.accounts().recover(&sig).unwrap();

        let secp = Secp256k1::new();
        let expected_address =
            PublicKey::from_secret_key(&secp, &provider.secret_key).to_eth_address();

        assert_eq!(recovered_address, expected_address);
    }
}
