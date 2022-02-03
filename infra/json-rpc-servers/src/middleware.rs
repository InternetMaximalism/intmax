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
                        start.elapsed().as_micros() as f64 / 1000_000.0
                    );
                    res
                })
                .instrument(span2),
        ))
    }
}
