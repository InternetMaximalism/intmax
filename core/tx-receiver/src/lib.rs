use ethereum_types::U256;
use fc_rpc_core::types::TransactionRequest;

use jsonrpc_core::{Error};

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
    fn validate_tx(&self, tx: &TransactionRequest) -> Result<(), jsonrpc_core::Error> {
        match tx.from {
            Some(_) => Some(()),
            None => return Err(Error::invalid_params("from is required")),
        };

        match tx.nonce {
            Some(_) => Some(()),
            None => return Err(Error::invalid_params("nonce is required")),
        };

        match tx.to {
            Some(_) => Some(()),
            None => return Err(Error::invalid_params("to is required")),
        };

        Ok(())
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
    use crate::{TxReceiver, TxReceiverTrait};
    use fc_rpc_core::types::TransactionRequest;
    use jsonrpc_core::{Error};
    use primitive_types::{H160, U256};

    #[test]
    fn fail_validate_tx_without_from() {
        let tx = TransactionRequest::default();
        let tx_receiver = TxReceiver::new();
        let err = tx_receiver.validate_tx(&tx).unwrap_err();

        assert_eq!(err, Error::invalid_params("from is required"));
    }

    #[test]
    fn fail_validate_tx_without_nonce() {
        let tx = TransactionRequest{
            from: Some(H160::random()),
            ..TransactionRequest::default()
        };

        let tx_receiver = TxReceiver::new();
        let err = tx_receiver.validate_tx(&tx).unwrap_err();

        assert_eq!(err, Error::invalid_params("nonce is required"));
    }

    #[test]
    fn fail_validate_tx_without_to() {
        let tx = TransactionRequest{
            from: Some(H160::random()),
            nonce: Some(U256::from(3000u32)),
            ..TransactionRequest::default()
        };

        let tx_receiver = TxReceiver::new();
        let err = tx_receiver.validate_tx(&tx).unwrap_err();

        assert_eq!(err, Error::invalid_params("to is required"));
    }

    fn success_validate_tx() {
        let tx = TransactionRequest{
            from: Some(H160::random()),
            nonce: Some(U256::from(3000u32)),
            to: Some(H160::random()),
            ..TransactionRequest::default()
        };

        let tx_receiver = TxReceiver::new();
        let err = tx_receiver.validate_tx(&tx).unwrap_err();

        assert_eq!(err, Error::invalid_params("to is required"));
    }

    #[test]
    fn success_estimate_gas() {
        // let tx = TransactionRequest::default();
        // let tx_receiver = TxReceiver::new();
        // let gas = tx_receiver.estimate_gas(&tx).unwrap();
        //
        // assert_eq!(gas, U256::from(0));
    }

    #[test]
    fn fail_estimate_gas() {
        // let tx = TransactionRequest::default();
        // let tx_receiver = TxReceiver::new();
        // let result = tx_receiver.estimate_gas(&tx).unwrap_err();
        // let expected = Err(Error::new(ErrorCode::InternalError));
        //
        // assert_eq!(result, expected);
    }
}
