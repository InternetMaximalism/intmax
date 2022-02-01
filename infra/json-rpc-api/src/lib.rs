use ethereum_types::H256;
use fc_rpc_core::types::TransactionRequest;
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;

#[rpc(server)]
pub trait EthApi {
    /// Sends transaction; will block waiting for signer to return the
    /// transaction hash.
    #[rpc(name = "eth_sendTransaction")]
    fn send_transaction(&self, _: TransactionRequest) -> BoxFuture<Result<H256>>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
