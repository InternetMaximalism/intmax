#[derive(Default)]
pub struct Runner {
    http_server: Option<http::Server>,
}

impl Runner {
    pub fn new() -> Runner {
        Runner::default()
    }

    pub fn regist_http_server(mut self, server: http::Server) -> Runner {
        self.http_server = Some(server);
        self
    }

    pub fn run(self) {
        println!("run!");
        if let Some(server) = self.http_server {
            server.wait();
        }
        println!("done")
    }
}

use intmax_config::Config;
use intmax_json_rpc_api::EthApi as EthApiT;
use intmax_rpc::EthApi;

pub fn gen_runner(config: &Config) -> Runner {
    let eth_api = EthApi::new();
    let rpc_handler = intmax_json_rpc_servers::rpc_handler(EthApiT::to_delegate(eth_api));
    let http_server = intmax_json_rpc_servers::start_http_server(
        &std::net::SocketAddr::new(
            config.rpc_server.ip.parse().expect("set valid ip address."),
            config.rpc_server.port,
        ),
        rpc_handler,
    )
    .expect("http server setup error.");
    Runner::new().regist_http_server(http_server)
}
