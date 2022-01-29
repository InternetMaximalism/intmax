use ethereum_types::{H256};
use fc_rpc_core::types::TransactionRequest;
use jsonrpc_core::{BoxFuture, Result, futures::future};

use intmax_json_rpc_api::EthApi as EthApiT;
use tx_receiver::{TxReceiver, TxReceiverTrait};

pub struct EthApi {
    tx_receiver: TxReceiver,
}

impl EthApi {
    pub fn new(tx_receiver: TxReceiver) -> EthApi {
        EthApi {
            tx_receiver
        }
    }
}

impl EthApiT for EthApi {
    fn send_transaction(&self, req: TransactionRequest) -> BoxFuture<Result<H256>> {
        match self.tx_receiver.validate_tx(&req) {
            Ok(()) => (),
            Err(e) => return Box::pin(future::err(e)),
        }

        Box::pin(async move { Ok(H256::zero()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fc_rpc_core::types::TransactionRequest;

    #[tokio::test]
    async fn it_works() {
        let tx_receiver = TxReceiver::new();
        let eth_api = EthApi::new(tx_receiver);

        let _res = eth_api.send_transaction(TransactionRequest::default());
    }
}
