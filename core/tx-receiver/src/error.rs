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
    /// Provided block range couldn't be resolved to a list of blocks.
    #[error("Cannot resolve a block range ['{:?}' ... '{:?}]. {}", .from, .to, .details)]
    InvalidBlockRange {
        /// Beginning of the block range.
        from: String,
        /// End of the block range.
        to: String,
        /// Details of the error message.
        details: String,
    },
    /// Provided count exceeds maximum value.
    #[error("count exceeds maximum value. value: {}, max: {}", .value, .max)]
    InvalidCount {
        /// Provided value
        value: u32,
        /// Maximum allowed value
        max: u32,
    },
    // Call to an unsafe RPC was denied.
    // #[error(transparent)]
    // UnsafeRpcCalled(#[from] crate::policy::UnsafeRpcError),
}

/// Base code for all state errors.
const BASE_ERROR: i64 = 4000;

impl From<Error> for rpc::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::InvalidBlockRange { .. } => rpc::Error {
                code: rpc::ErrorCode::ServerError(BASE_ERROR + 1),
                message: format!("{}", e),
                data: None,
            },
            Error::InvalidCount { .. } => rpc::Error {
                code: rpc::ErrorCode::ServerError(BASE_ERROR + 2),
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
    use jsonrpc_core as rpc;
    use jsonrpc_core::{BoxFuture, Result as CoreResult};
    use ethereum_types::H256;
    

    fn raise_invalid_count() -> Result<H256, Error> {
        let e = Error::InvalidCount { value: 0, max: 0 };
        Err(e)
    }

    fn raiser() -> CoreResult<H256> {
        let h = raise_invalid_count()?;
        Ok(h)
    }


    fn send_transaction() -> BoxFuture<CoreResult<H256>> {
        let h = match raiser() {
            Ok(h) => h,
            Err(e) => return Box::pin(async move { Err(e) })
        };

        Box::pin(async move {Ok(h)})
    }

    #[tokio::test]
    async fn success_type_conversion_into_core() {
        let res = send_transaction().await;
        let e = res.err().unwrap();

        assert_eq!(e.code, rpc::ErrorCode::ServerError(4002));
        assert_eq!(e.message, String::from("count exceeds maximum value. value: 0, max: 0"));
    }
}
