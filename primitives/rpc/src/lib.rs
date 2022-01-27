use ethereum_types::H256;
use fc_rpc_core::types::TransactionRequest;
use jsonrpc_core::{BoxFuture, Result};

use intmax_json_rpc_api::EthApi as EthApiT;

pub struct EthApi;

impl EthApi {
    pub fn new() -> EthApi {
        EthApi {}
    }
}

impl EthApiT for EthApi {
    fn send_transaction(&self, _: TransactionRequest) -> BoxFuture<Result<H256>> {
        Box::pin(async move { Ok(H256::zero()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fc_rpc_core::types::TransactionRequest;
    #[test]
    fn it_works() {
        let eth_api = EthApi::new();
        eth_api.send_transaction(TransactionRequest::default());
    }
}
