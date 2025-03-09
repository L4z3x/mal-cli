// auth config
pub mod oauth_config;

// app config
pub mod app_config;

// pub use app_config::AppConfig;
pub use oauth_config::AuthConfig;

use std::path::PathBuf;

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "mal-tui";

const DEFAULT_PORT: u16 = 7878;
const DEFAULT_USER_AGENT: &str = "mal-tui";
const OAUTH_FILE: &str = "oauth2.yml";
const TOKEN_CACHE_FILE: &str = ".mal_token_cache.json";

const _CONFIG_FILE: &str = "config.yml";

#[derive(Debug)]
pub enum ConfigError {
    /// Represents an invalid config file
    EmptyConfig,
    /// Represents a failure to read from input
    ReadError,
    /// Represents a nonexistent path error
    PathError,
    /// Represents a serde_yaml parse error
    ParseError(serde_yaml::Error),
    /// Represents all other failures
    IOError(std::io::Error),
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ConfigError::EmptyConfig => None,
            ConfigError::ReadError => None,
            ConfigError::PathError => None,
            ConfigError::ParseError(_) => None,
            ConfigError::IOError(_) => None,
        }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ConfigError::EmptyConfig => write!(f, "Source contains no data"),
            ConfigError::ReadError => write!(f, "Could not read file"),
            ConfigError::PathError => write!(f, "Path not found"),
            ConfigError::ParseError(ref err) => err.fmt(f),
            ConfigError::IOError(ref err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IOError(err)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigError::ParseError(err)
    }
}

pub struct ConfigPaths {
    pub config_file_path: PathBuf,
    pub auth_cache_path: PathBuf,
}
