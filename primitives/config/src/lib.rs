use config::{Config as ConfigRs, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RpcServerConfig {
    pub port: u32,
    pub ip: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rpc_server: RpcServerConfig,
}

pub enum ConfigKind {
    TEST,
    DEV,
    MAIN,
}

impl Config {
    fn new(kind: ConfigKind) -> Result<Self, ConfigError> {
        let name = match kind {
            ConfigKind::TEST => "config_test",
            ConfigKind::DEV => "config_dev",
            ConfigKind::MAIN => "config",
        };
        let mut s = ConfigRs::new();
        println!("new");
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
        assert_eq!(config.rpc_server.port, 8081);
        assert_eq!(config.rpc_server.ip, "127.0.0.1");
    }
}
