use config::{Config as ConfigRs, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RpcServerConfig {
    pub port: u16,
    pub ip: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub http_server: RpcServerConfig,
    pub ws_server: RpcServerConfig,
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
            ConfigKind::DEV => "config",
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
    }
}
