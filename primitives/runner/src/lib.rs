#[derive(Default)]
pub struct Runner {
    http_server: Option<http::Server>,
    ws_server: Option<ws::Server>,
}

impl Runner {
    pub fn new() -> Runner {
        Runner::default()
    }

    pub fn regist_http_server(mut self, server: http::Server) -> Runner {
        self.http_server = Some(server);
        self
    }

    pub async fn run(self) {
        println!("run!");
        let mut tasks = Vec::new();

        // regist task: http server.
        if let Some(server) = self.http_server {
            tasks.push(tokio::spawn(async move { server.wait() }));
        };

        // regist tasks: ws server
        if let Some(server) = self.ws_server {
            tasks.push(tokio::spawn(async move {
                server.wait().expect("ws server setup error.")
            }));
        };

        println!("done");
        futures::future::join_all(tasks.into_iter()).await;
    }
}

use intmax_config::Config;
use intmax_json_rpc_api::EthApi as EthApiT;
use intmax_rpc::EthApi;
use tx_receiver::TxReceiver;

pub fn gen_runner(config: &Config) -> Runner {
    let tx_receiver = TxReceiver::new();
    let eth_api = EthApi::new(tx_receiver);

    let gen_handler = |apis| intmax_json_rpc_servers::rpc_handler(EthApiT::to_delegate(apis));
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt().init();

    let rpc_handler = intmax_json_rpc_servers::rpc_handler(EthApiT::to_delegate(eth_api));
    let http_server = intmax_json_rpc_servers::start_http_server(
        &std::net::SocketAddr::new(
            config
                .http_server
                .ip
                .parse()
                .expect("set valid ip address."),
            config.http_server.port,
        ),
        gen_handler(EthApi::new(TxReceiver::new())),
    )
    .expect("http server setup error.");
    let ws_server = intmax_json_rpc_servers::start_ws_server(
        &std::net::SocketAddr::new(
            config.ws_server.ip.parse().expect("set valid ip address."),
            config.ws_server.port,
        ),
        rpc_handler,
    )
    .expect("http server setup error.");
    Runner::new().regist_http_server(http_server)
}
