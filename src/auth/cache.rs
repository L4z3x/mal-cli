use super::OAuth;
use crate::config::oauth_config::AuthConfig;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

pub fn cache_auth(auth: &OAuth) {
    let auth_path = AuthConfig::get_paths().unwrap().auth_cache_path;

    let mut auth_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(auth_path)
        .unwrap();

    let cached_auth = serde_json::to_string(auth).unwrap();

    write!(auth_file, "{}", cached_auth).unwrap();
}

pub fn load_cached_auth() -> Option<OAuth> {
    let auth_path = AuthConfig::get_paths().unwrap().auth_cache_path;
    let mut auth_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(auth_path)
        .unwrap();

    let mut cached_string = String::new();

    auth_file.read_to_string(&mut cached_string).unwrap();

    let cached_auth: OAuth = match serde_json::from_str(&cached_string) {
        Ok(s) => s,
        Err(_) => return None,
    };

    Some(cached_auth)
}

pub fn delete_cached_auth() {
    let auth_path = AuthConfig::get_paths().unwrap().config_file_path;
    let _ = std::fs::remove_file(auth_path);
}
