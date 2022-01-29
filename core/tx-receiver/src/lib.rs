use ethereum_types::{U256};
use fc_rpc_core::types::TransactionRequest;

pub trait TxReceiverTrait {
    fn validate_tx(&self, tx: &TransactionRequest) -> Result<(), jsonrpc_core::Error>;
    fn verify_zkp(&self, tx: &TransactionRequest) -> Result<(), jsonrpc_core::Error>;
    fn estimate_gas(&self, tx: &TransactionRequest) -> Result<U256, jsonrpc_core::Error>;
    fn put_tx_into_mempool(&self, tx: &TransactionRequest) -> Result<(), jsonrpc_core::Error>;
}

pub struct TxReceiver;

impl TxReceiver {
    pub fn new() -> Self {
        TxReceiver {}
    }
}

impl TxReceiverTrait for TxReceiver {
    fn validate_tx(&self, _tx: &TransactionRequest) -> Result<(), jsonrpc_core::Error> {
        todo!()
    }

    fn verify_zkp(&self, _tx: &TransactionRequest) -> Result<(), jsonrpc_core::Error> {
        todo!()
    }
    fn estimate_gas(&self, _tx: &TransactionRequest) -> Result<U256, jsonrpc_core::Error> {
        todo!()
    }
    fn put_tx_into_mempool(&self, _tx: &TransactionRequest) -> Result<(), jsonrpc_core::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use ethereum_types::U256;
    use fc_rpc_core::types::TransactionRequest;
    use jsonrpc_core::{Error, ErrorCode};
    use crate::{TxReceiver, TxReceiverTrait};

    #[test]
    fn success_estimate_gas() {
        let tx = TransactionRequest::default();
        let tx_receiver = TxReceiver::new();
        let gas = tx_receiver.estimate_gas(&tx).unwrap();

        assert_eq!(gas, U256::from(0));
    }

    #[test]
    fn fail_estimate_gas() {
        let tx = TransactionRequest::default();
        let tx_receiver = TxReceiver::new();
        let result = tx_receiver.estimate_gas(&tx).unwrap_err();
        let expected = Err(Error::new(ErrorCode::InternalError));

        assert_eq!(result, expected);
    }
}
