use jsonrpc_core::IoHandlerExtension;

pub type RpcHandler = jsonrpc_core::IoHandler;

/// Construct rpc `IoHandler`
pub fn rpc_handler(extension: impl IoHandlerExtension<()>) -> RpcHandler {
    let mut io = RpcHandler::new();
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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
