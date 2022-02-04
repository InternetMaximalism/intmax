use jsonrpc_core::IoHandlerExtension;
use jsonrpc_core::MetaIoHandler;

use crate::middleware::{Meta, TracingMiddleware};

pub type RpcHandler = jsonrpc_core::MetaIoHandler<Meta, TracingMiddleware>;

mod middleware;

/// Construct rpc `IoHandler`
pub fn rpc_handler(
    extension: impl IoHandlerExtension<Meta>,
) -> MetaIoHandler<Meta, TracingMiddleware> {
    let mut io = MetaIoHandler::with_middleware(TracingMiddleware::default());
    extension.augment(&mut io);

    // add an endpoint to list all available methods.
    let mut methods = io.iter().map(|x| x.0.clone()).collect::<Vec<String>>();
    io.add_method("rpc_methods", {
        methods.sort();
        let methods = serde_json::to_value(&methods)
            .expect("Serialization of Vec<String> is infallible; qed");

        move |_| {
            let methods = methods.clone();
            async move {
                Ok(serde_json::json!({
                    "version": 1,
                    "methods": methods,
                }))
            }
        }
    });

    io
}

pub fn start_http_server(
    addr: &std::net::SocketAddr,
    io: RpcHandler,
) -> std::io::Result<http::Server> {
    println!("server address: {}", addr);
    http::ServerBuilder::new(io).threads(1).start_http(addr)
}

#[cfg(test)]
mod tests {
    use crate::middleware::TracingMiddleware;
    use crate::{rpc_handler, start_http_server};
    use jsonrpc_core::MetaIoHandler;
    use reqwest;
    use reqwest::StatusCode;

    #[test]
    fn success_server_start() {
        let io = MetaIoHandler::with_middleware(TracingMiddleware::default());

        let rpc_handler = rpc_handler(io);
        let http_server = start_http_server(
            &std::net::SocketAddr::new("127.0.0.1".parse().expect("set valid ip address."), 8080),
            rpc_handler,
        );

        assert!(http_server.is_ok());

        let client_fut = async move {
            let c = reqwest::Client::new();
            let res = c
                .post("http://127.0.0.1:8080")
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "rpc_methods",
                    "id": 1
                }))
                .send()
                .await
                .expect("should success request");
            assert_eq!(res.status(), StatusCode::OK);
        };

        tokio::runtime::Runtime::new().unwrap().block_on(client_fut);
    }
}
