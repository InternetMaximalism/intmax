use ethereum_types::H256;
use fc_rpc_core::types::TransactionRequest;
use jsonrpc_core::{BoxFuture, Result};
use tracing::info_span;

use intmax_json_rpc_api::EthApi as EthApiT;
use tx_receiver::{TxReceiver, TxReceiverTrait};

#[derive(Debug)]
pub struct EthApi {
    tx_receiver: TxReceiver,
}

mod error;

impl EthApi {
    pub fn new(tx_receiver: TxReceiver) -> EthApi {
        EthApi { tx_receiver }
    }
}

impl EthApiT for EthApi {
    fn send_transaction(&self, req: TransactionRequest) -> BoxFuture<Result<H256>> {
        let _guard = info_span!("send_transaction").entered();

        match self.tx_receiver.validate_tx(&req) {
            Ok(()) => (),
            Err(e) => return Box::pin(async move { Err(e.into()) }),
        }

        Box::pin(async move { Ok(H256::zero()) })
    }
}

#[cfg(test)]
mod tests {
    use fc_rpc_core::types::TransactionRequest;
    use primitive_types::{H160, U256};

    use super::*;

    #[tokio::test]
    async fn success_send_transaction() {
        let tx_receiver = TxReceiver::new();
        let eth_api = EthApi::new(tx_receiver);
        let tx = TransactionRequest {
            from: Some(H160::random()),
            nonce: Some(U256::from(3000u32)),
            to: Some(H160::random()),
            ..TransactionRequest::default()
        };

        let _res = eth_api.send_transaction(tx);
    }

    #[tokio::test]
    async fn fail_send_transaction() {
        let tx_receiver = TxReceiver::new();
        let eth_api = EthApi::new(tx_receiver);

        let _res = eth_api.send_transaction(TransactionRequest::default());
    }
}
