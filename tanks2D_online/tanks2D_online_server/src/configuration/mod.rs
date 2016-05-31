
pub const PATH_TO_CONFIG_FILE: &'static str = "server_config.json";

mod server_config;

pub use self::server_config::{ ServerConfig, ServerConfigurationError, Result };
