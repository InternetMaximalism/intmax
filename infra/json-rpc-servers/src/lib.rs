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

pub fn start_ws_server(addr: &std::net::SocketAddr, io: RpcHandler) -> std::io::Result<ws::Server> {
    println!("server address: {}", addr);
    ws::ServerBuilder::new(io)
        .start(addr)
        .map_err(|err| match err {
            ws::Error::Io(io) => io,
            ws::Error::ConnectionClosed => std::io::ErrorKind::BrokenPipe.into(),
            er => {
                println!("error: {:?}", er);
                // output error log.
                std::io::ErrorKind::Other.into()
            }
        })
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::middleware::TracingMiddleware;
    use jsonrpc_core::{MetaIoHandler, Params};
    use jsonrpc_core_client::{transports, RawClient, RpcChannel};
    use reqwest;
    use reqwest::StatusCode;

    #[test]
    fn success_http_server_start() {
        let io = MetaIoHandler::with_middleware(TracingMiddleware::default());

        let rpc_handler = rpc_handler(io);
        let server = start_http_server(
            &std::net::SocketAddr::new("127.0.0.1".parse().expect("set valid ip address."), 8080),
            rpc_handler,
        );

        assert!(server.is_ok());

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

    #[test]
    fn success_ws_server_start() {
        let io = MetaIoHandler::with_middleware(TracingMiddleware::default());

        let rpc_handler = rpc_handler(io);
        let server = start_ws_server(
            &std::net::SocketAddr::new("127.0.0.1".parse().expect("set valid ip address."), 3030),
            rpc_handler,
        );
        assert!(server.is_ok());

        let client_fut = transports::ws::connect::<RpcChannel>(
            &url::Url::parse("ws://127.0.0.1:3030").expect("set valid ip address."),
        );

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let sender: RawClient = client_fut.await.expect("happen rpc error.").into();
                let res = sender.call_method("rpc_methods", Params::None).await;
                assert!(res.is_ok());
            });
    }
}
