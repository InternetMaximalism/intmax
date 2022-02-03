use error::Error;
use ethereum_types::U256;
use fc_rpc_core::types::TransactionRequest;

mod error;

pub trait TxReceiverTrait {
    fn validate_tx(&self, tx: &TransactionRequest) -> Result<(), Error>;
    fn verify_zkp(&self, tx: &TransactionRequest) -> Result<(), Error>;
    fn estimate_gas(&self, tx: &TransactionRequest) -> Result<U256, Error>;
    fn put_tx_into_mempool(&self, tx: &TransactionRequest) -> Result<(), Error>;
}

pub struct TxReceiver;

impl TxReceiver {
    pub fn new() -> Self {
        TxReceiver {}
    }
}

impl TxReceiverTrait for TxReceiver {
    fn validate_tx(&self, tx: &TransactionRequest) -> Result<(), Error> {
        match tx.from {
            Some(_) => Some(()),
            None => {
                return Err(Error::invalid_params("from is required"));
            }
        };

        match tx.nonce {
            Some(_) => Some(()),
            None => {
                return Err(Error::invalid_params("nonce is required"));
            }
        };

        match tx.to {
            Some(_) => Some(()),
            None => {
                return Err(Error::invalid_params("to is required"));
            }
        };

        Ok(())
    }

    fn verify_zkp(&self, _tx: &TransactionRequest) -> Result<(), Error> {
        todo!()
    }
    fn estimate_gas(&self, _tx: &TransactionRequest) -> Result<U256, Error> {
        todo!()
    }
    fn put_tx_into_mempool(&self, _tx: &TransactionRequest) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use fc_rpc_core::types::TransactionRequest;
    use jsonrpc_core::error::Error as CoreError;
    use primitive_types::{H160, U256};

    use crate::{TxReceiver, TxReceiverTrait};

    #[test]
    fn fail_validate_tx_without_from() {
        let tx = TransactionRequest::default();
        let tx_receiver = TxReceiver::new();
        let err: CoreError = tx_receiver.validate_tx(&tx).unwrap_err().into();

        assert_eq!(err, CoreError::invalid_params("from is required"));
    }

    #[test]
    fn fail_validate_tx_without_nonce() {
        let tx = TransactionRequest {
            from: Some(H160::random()),
            ..TransactionRequest::default()
        };

        let tx_receiver = TxReceiver::new();
        let err: CoreError = tx_receiver.validate_tx(&tx).unwrap_err().into();

        assert_eq!(err, CoreError::invalid_params("nonce is required"));
    }

    #[test]
    fn fail_validate_tx_without_to() {
        let tx = TransactionRequest {
            from: Some(H160::random()),
            nonce: Some(U256::from(3000u32)),
            ..TransactionRequest::default()
        };

        let tx_receiver = TxReceiver::new();
        let err: CoreError = tx_receiver.validate_tx(&tx).unwrap_err().into();

        assert_eq!(err, CoreError::invalid_params("to is required"));
    }

    #[test]
    fn success_validate_tx() {
        let tx = TransactionRequest {
            from: Some(H160::random()),
            nonce: Some(U256::from(3000u32)),
            to: Some(H160::random()),
            ..TransactionRequest::default()
        };

        let tx_receiver = TxReceiver::new();
        let is_ok = tx_receiver.validate_tx(&tx).is_ok();

        assert!(is_ok);
    }

    #[test]
    #[should_panic]
    fn panic_verify_zkp() {
        let tx = TransactionRequest::default();
        let tx_receiver = TxReceiver::new();
        tx_receiver.verify_zkp(&tx).unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_estimate_gas() {
        let tx = TransactionRequest::default();
        let tx_receiver = TxReceiver::new();
        tx_receiver.estimate_gas(&tx).unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_put_tx_into_mempool() {
        let tx = TransactionRequest::default();
        let tx_receiver = TxReceiver::new();
        tx_receiver.put_tx_into_mempool(&tx).unwrap();
    }
}
