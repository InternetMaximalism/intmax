use jsonrpc_core::futures_util::{future::Either, FutureExt};
use jsonrpc_core::*;
use std::future::Future;
use std::sync::atomic::AtomicUsize;
use std::time::Instant;
use tracing::{info, info_span};
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct Meta(usize);

impl Metadata for Meta {}

#[derive(Default)]
pub struct TracingMiddleware(AtomicUsize);

impl Middleware<Meta> for TracingMiddleware {
    type Future = FutureResponse;
    type CallFuture = middleware::NoopCallFuture;

    fn on_request<F, X>(&self, request: Request, meta: Meta, next: F) -> Either<Self::Future, X>
    where
        F: FnOnce(Request, Meta) -> X + Send,
        X: Future<Output = Option<Response>> + Send + 'static,
    {
        let start = Instant::now();
        let req_id = Uuid::new_v4().to_string();
        let _guard = info_span!("jsonrpc", reques_id = %req_id).entered();
        let span2 = info_span!("rpc calling", reques_id = %req_id);
        info!(
            "request: \n{}",
            serde_json::to_string_pretty(&request).unwrap_or_default()
        );

        Either::Left(Box::pin(
            next(request, meta)
                .map(move |res| {
                    if let Some(v) = &res {
                        info!(
                            "response: \n{}",
                            serde_json::to_string_pretty(&v).unwrap_or_default()
                        );
                    }
                    info!(
                        "Processing took: {:.6}sec",
                        start.elapsed().as_micros() as f64 / 1_000_000.0
                    );
                    res
                })
                .instrument(span2),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::middleware::{Meta, TracingMiddleware};
    use http::jsonrpc_core::Value;
    use jsonrpc_core::{MetaIoHandler, Params};

    #[test]
    fn success_middleware_with_some_method() {
        // for coverage
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();

        let mut io = MetaIoHandler::with_middleware(TracingMiddleware::default());

        io.add_method_with_meta("say_hello", |_params: Params, meta: Meta| async move {
            Ok(Value::String(format!("Hello World: {}", meta.0)))
        });

        let request = r#"{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23], "id": 1}"#;
        let response = r#"{"jsonrpc":"2.0","result":"Hello World: 5","id":1}"#;
        assert_eq!(
            io.handle_request_sync(request, Meta(5)),
            Some(response.to_owned())
        );

        let m = Meta(0);
        println!("{:?}", m);
    }

    #[test]
    #[should_panic]
    fn success_middleware_without_response() {
        let mut io = MetaIoHandler::with_middleware(TracingMiddleware::default());

        io.add_method_with_meta("say_hello", |_params: Params, _meta: Meta| async move {
            panic!("test");
        });

        let request = r#"{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23], "id": 1}"#;
        io.handle_request_sync(request, Meta(5));
    }
}
