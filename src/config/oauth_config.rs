use super::*;
use figlet_rs::FIGfont;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{
    fs,
    io::{stdin, Write},
    path::Path,
};
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct AuthConfig {
    pub client_id: String,
    pub user_agent: Option<String>,
    pub port: Option<u16>,
}

impl AuthConfig {
    // TODO: Strip whitespace from user_agent as it can cause code to panic
    pub fn load() -> Result<Self, ConfigError> {
        let paths = Self::get_paths()?;
        if paths.config_file_path.exists() {
            let config_string = fs::read_to_string(&paths.config_file_path)?;
            let config_yml: AuthConfig = serde_yaml::from_str(&config_string)?;

            Ok(config_yml)
        } else {
            let standard_font = FIGfont::standard().unwrap();
            let figlet = standard_font.convert("MAL-CLI");
            let banner = figlet.unwrap().to_string();
            println!("{}", banner);
            println!(
                "Config will be saved to {}",
                paths.config_file_path.display()
            );

            println!("\nHow to get setup:\n");

            let instructions = [
                "Go to the myanimelist api page - https://myanimelist.net/apiconfig",
                "Click `Create ID` and create an app",
                &format!(
                    "Add `http://127.0.0.1:{}` to the Redirect URIs",
                    DEFAULT_PORT
                ),
                "You are now ready to authenticate with myanimelist!",
            ];

            let mut number = 1;
            for item in instructions.iter() {
                println!("   {}. {}", number, item);
                number += 1;
            }

            let mut client_id = String::new();
            loop {
                println!("\nEnter your client ID: ");
                stdin().read_line(&mut client_id)?;
                let trimmed_client_id = client_id.trim().to_string();
                if trimmed_client_id.len() == 32
                    && trimmed_client_id.chars().all(|c| c.is_ascii_hexdigit())
                {
                    client_id = trimmed_client_id;
                    break;
                } else {
                    println!("Invalid client ID format. try again");
                    client_id.clear();
                }
            }
            let mut user_agent = String::new();
            println!("\nEnter User Agent (default {}): ", DEFAULT_USER_AGENT);
            stdin().read_line(&mut user_agent)?;

            let user_agent = match user_agent.trim().len() {
                0 => DEFAULT_USER_AGENT.to_string(),
                _ => user_agent.trim().to_string(),
            };

            let mut port = String::new();
            println!("\nEnter port of redirect uri (default {}): ", DEFAULT_PORT);
            stdin().read_line(&mut port)?;
            let port = port.trim().parse::<u16>().unwrap_or(DEFAULT_PORT);

            let config_yml = AuthConfig {
                client_id,
                user_agent: Some(user_agent),
                port: Some(port),
            };

            let content_yml = serde_yaml::to_string(&config_yml)?;

            let mut new_config = fs::File::create(&paths.config_file_path)?;
            write!(new_config, "{}", content_yml)?;

            Ok(config_yml)
        }
    }

    pub fn get_redirect_uri(&self) -> String {
        format!("127.0.0.1:{}", self.get_port())
    }

    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(DEFAULT_PORT)
    }

    pub fn get_user_agent(&self) -> String {
        match &self.user_agent {
            Some(s) => s.clone(),
            None => DEFAULT_USER_AGENT.to_string(),
        }
    }

    pub fn get_paths() -> Result<ConfigPaths, ConfigError> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

                if !home_config_dir.exists() {
                    fs::create_dir(&home_config_dir)?;
                }

                if !app_config_dir.exists() {
                    fs::create_dir(&app_config_dir)?;
                }

                let config_file_path = &app_config_dir.join(OAUTH_FILE);
                let token_cache_path = &app_config_dir.join(TOKEN_CACHE_FILE);

                let paths = ConfigPaths {
                    config_file_path: config_file_path.to_path_buf(),
                    auth_cache_path: token_cache_path.to_path_buf(),
                };

                Ok(paths)
            }
            None => Err(ConfigError::PathError),
        }
    }
}
