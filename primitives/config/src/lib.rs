use config::{Config as ConfigRs, ConfigError, File};
use serde_derive::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct RpcServerConfig {
    pub port: u16,
    pub ip: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Scheme {
    Http,
    Https,
    Ws,
    Wss,
}

impl Default for Scheme {
    fn default() -> Self {
        Self::Http
    }
}

impl Display for Scheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Scheme::Http => write!(f, "http"),
            Scheme::Https => write!(f, "https"),
            Scheme::Ws => write!(f, "ws"),
            Scheme::Wss => write!(f, "wss"),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct EthConfig {
    pub committer_key: String,
    pub port: u16,
    pub host: String,
    pub scheme: Scheme,
}

impl EthConfig {
    pub fn node_url(&self) -> String {
        format!("{}://{}:{}", self.scheme, self.host, self.port)
    }

    pub fn is_http(&self) -> bool {
        self.scheme == Scheme::Http || self.scheme == Scheme::Https
    }

    pub fn is_ws(&self) -> bool {
        self.scheme == Scheme::Ws || self.scheme == Scheme::Wss
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub http_server: RpcServerConfig,
    pub ws_server: RpcServerConfig,
    pub eth_server: EthConfig,
}

pub enum ConfigKind {
    TEST,
    DEV,
    MAIN,
}

impl Config {
    pub fn new(kind: ConfigKind) -> Result<Self, ConfigError> {
        let name = match kind {
            ConfigKind::TEST => "config_test",
            ConfigKind::DEV => "config_dev",
            ConfigKind::MAIN => "config",
        };
        let mut s = ConfigRs::new();
        println!(
            "root: {}",
            project_root::get_project_root().unwrap().to_str().unwrap()
        );
        s.merge(File::with_name(&format!(
            "{}/res/{}",
            project_root::get_project_root().unwrap().to_str().unwrap(),
            name
        )))?;
        println!("merged: {:?}", s);
        s.try_into()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn new_conifg() {
        let config = Config::new(ConfigKind::TEST).expect("error new config");
        assert_eq!(config.http_server.port, 8081);
        assert_eq!(config.http_server.ip, "127.0.0.1");
        assert_eq!(config.ws_server.port, 3030);
        assert_eq!(config.ws_server.ip, "127.0.0.1");
        assert_eq!(
            config.eth_server.committer_key,
            "10d18ee85b1a2e1d4b47feed91074a6bb4a17b55005144338208a0be031752d3"
        );
        assert_eq!(config.eth_server.port, 8545);
        assert_eq!(config.eth_server.host, "127.0.0.1");
        assert_eq!(config.eth_server.scheme, Scheme::Http);
        assert_eq!(config.eth_server.node_url(), "http://127.0.0.1:8545");
        assert!(config.eth_server.is_http());
        assert!(!config.eth_server.is_ws());

        assert_eq!(format!("{}", Scheme::Http), "http");
        assert_eq!(format!("{}", Scheme::Https), "https");
        assert_eq!(format!("{}", Scheme::Ws), "ws");
        assert_eq!(format!("{}", Scheme::Wss), "wss");

        let config = EthConfig {
            scheme: Scheme::Ws,
            ..EthConfig::default()
        };

        assert!(config.is_ws());
        assert!(!config.is_http());
    }
}
