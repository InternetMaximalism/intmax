use ethereum_types::{H160, H256};
use jsonrpc_core as rpc;

/// State RPC Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// State RPC future Result type.
pub type FutureResult<T> = jsonrpc_core::BoxFuture<Result<T>>;

pub fn internal<E: ::std::fmt::Debug>(e: E) -> rpc::Error {
    jsonrpc_core::Error {
        code: rpc::ErrorCode::InternalError,
        message: "Unknown error occurred".into(),
        data: Some(format!("{:?}", e).into()),
    }
}

/// State RPC errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Client error.
    #[error("Client error: {}", .0)]
    Client(#[from] Box<dyn std::error::Error + Send>),

    /// If StateDiff has already been applied
    #[error("StateDiff({}) has already been used", .state_diff)]
    InvalidStateDiff {
        /// State diff already used
        state_diff: H256,
    },

    /// If OneTimeAddress is an address that has already been used
    #[error("OneTimeAddress({}) has already been used", .one_time_address)]
    InvalidOneTimeAddress {
        /// One time address already used
        one_time_address: H160,
    },

    /// Error when user_state_proof is invalid.
    #[error("UserStateProof({}) is invalid", .user_state_proof)]
    InvalidUserStateZKP {
        /// Invalid user state proof
        user_state_proof: String,
    },

    /// Error when signed_tx_proof is invalid.
    #[error("SignedTxProof({}) is invalid", .signed_tx_proof)]
    InvalidSignedTxZKP {
        /// Invalid sign proof
        signed_tx_proof: String,
    },
}

impl From<Error> for rpc::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::InvalidStateDiff { .. }
            | Error::InvalidOneTimeAddress { .. }
            | Error::InvalidUserStateZKP { .. }
            | Error::InvalidSignedTxZKP { .. } => rpc::Error {
                code: rpc::ErrorCode::InvalidParams,
                message: format!("{}", e),
                data: None,
            },
            e => internal(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use ethereum_types::{H160, H256};
    use jsonrpc_core as rpc;
    use jsonrpc_core::{BoxFuture, Result as CoreResult};
    use std::fmt;

    #[derive(Debug, Clone, thiserror::Error)]
    struct SampleError;

    impl fmt::Display for SampleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "SampleError is here!")
        }
    }

    fn raise_invalid_one_time_address() -> Result<H256, Error> {
        let e = Error::InvalidOneTimeAddress {
            one_time_address: H160::zero(),
        };

        Err(e)
    }

    fn raise_client() -> Result<H256, Error> {
        Err(Error::Client(Box::new(SampleError {})))
    }

    fn raiser_invalid_count() -> CoreResult<H256> {
        let h = raise_invalid_one_time_address()?;
        Ok(h)
    }

    fn raiser_client() -> CoreResult<H256> {
        let h = raise_client()?;
        Ok(h)
    }

    fn send_transaction() -> BoxFuture<CoreResult<H256>> {
        let h = match raiser_invalid_count() {
            Ok(h) => h,
            Err(e) => return Box::pin(async move { Err(e) }),
        };

        Box::pin(async move { Ok(h) })
    }

    fn send_transaction2() -> BoxFuture<CoreResult<H256>> {
        let h = match raiser_client() {
            Ok(h) => h,
            Err(e) => return Box::pin(async move { Err(e) }),
        };

        Box::pin(async move { Ok(h) })
    }

    #[tokio::test]
    async fn success_type_conversion_into_core_err() {
        let res = send_transaction().await;
        let e = res.err().unwrap();

        assert_eq!(e.code, rpc::ErrorCode::InvalidParams);
        assert_eq!(
            e.message,
            String::from("OneTimeAddress(0x0000…0000) has already been used")
        );
    }

    #[tokio::test]
    async fn success_type_conversion_into_core2() {
        let res = send_transaction2().await;
        let e = res.err().unwrap();

        assert_eq!(e.code, rpc::ErrorCode::InternalError);
        assert_eq!(e.message, String::from("Unknown error occurred"));

        assert_eq!(e.data.unwrap(), String::from("Client(SampleError)"));
    }

    #[test]
    fn success_type_conversion_into_core_all() {
        // Error::InvalidStateDiff
        let e = Error::InvalidStateDiff {
            state_diff: H256::zero(),
        };

        let e2: rpc::Error = e.into();

        assert_eq!(e2.message, "StateDiff(0x0000…0000) has already been used");
        assert_eq!(e2.code, rpc::ErrorCode::InvalidParams);

        // Error::InvalidOneTimeAddress
        let e = Error::InvalidOneTimeAddress {
            one_time_address: H160::zero(),
        };

        let e2: rpc::Error = e.into();

        assert_eq!(
            e2.message,
            "OneTimeAddress(0x0000…0000) has already been used"
        );
        assert_eq!(e2.code, rpc::ErrorCode::InvalidParams);

        // Error::InvalidUserStateZKP
        let e = Error::InvalidUserStateZKP {
            user_state_proof: String::from("0x0101010101"),
        };

        let e2: rpc::Error = e.into();

        assert_eq!(e2.message, "UserStateProof(0x0101010101) is invalid");
        assert_eq!(e2.code, rpc::ErrorCode::InvalidParams);

        // Error::InvalidUserStateZKP
        let e = Error::InvalidSignedTxZKP {
            signed_tx_proof: String::from("0x0101010101"),
        };

        let e2: rpc::Error = e.into();

        assert_eq!(e2.message, "SignedTxProof(0x0101010101) is invalid");
        assert_eq!(e2.code, rpc::ErrorCode::InvalidParams);
    }
}
