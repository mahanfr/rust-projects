use core::fmt;
use std::{collections::HashMap, fmt::Display, fs, path::Path};
use ini::Ini;
use crate::DEV;

pub fn string_to_bool(string: &str) -> Result<bool, ConfigError> {
    if string == "true" {
        Ok(true)
    } else if string == "false" {
        Ok(false)
    } else {
        Err(ConfigError::ConfigValueError)
    }
}

pub struct AppConfig {
    pub debug: bool,
    pub server_addr: &'static str,
    pub server_port: u16,
    pub admin_addr: &'static str,
    pub admin_port: u16,
    pub other: HashMap<&'static str, &'static str>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            debug: false,
            server_addr: "127.0.0.1",
            server_port: 3000,
            admin_addr: "127.0.0.1",
            admin_port: 3001,
            other: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    ConfigFileError,
    ConfigValueError,
    INIError(ini::Error),
    INIValueError(std::num::ParseIntError)
}
impl ConfigError {
    #[inline]
    fn description_str(&self) -> &'static str {
        match *self {
            ConfigError::ConfigFileError => "Invalid config file formatting",
            ConfigError::ConfigValueError => "Invalid config value",
            ConfigError::INIError(_) => "Invalid INI file",
            ConfigError::INIValueError(_) => "Invalid Value for the config"
        }
    }
}

impl From<ini::Error> for ConfigError{
    fn from(e: ini::Error) -> Self {
        ConfigError::INIError(e)
    }
}

impl From<std::num::ParseIntError> for ConfigError{
    fn from(e: std::num::ParseIntError) -> Self {
        ConfigError::INIValueError(e)
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.description_str())
    }
}

pub fn get_app_config() -> Result<AppConfig, ConfigError> {
    let mut config: AppConfig = AppConfig::default();
    let path: &str;
    if DEV == true {
        path = "dev/config.ini";
        if !Path::new("dev/").exists() {
            match fs::create_dir("dev") {
                Ok(_) => {}
                Err(_) => {
                    return Err(ConfigError::ConfigFileError);
                }
            }
        }
    } else {
        path = "config.ini";
    }
    if !Path::new(path).exists() {
        match fs::copy("templates/config.ini", path) {
            Ok(_) => {}
            Err(_) => {
                return Err(ConfigError::ConfigFileError);
            }
        }
    }
    let conf = Box::leak(Box::new(
        Ini::load_from_file(path)?,
    ));
    
    match conf.section(Some("general")){
        Some(sec) => {
            config.debug = string_to_bool(sec.get("debug").unwrap())?;
        },
        None => todo!(),
    };

    match conf.section(Some("server")){
        Some(sec) => {
            config.server_addr = sec.get("address").unwrap();
            config.server_port = sec
                .get("port")
                .unwrap()
                .to_string()
                .parse::<u16>()
                .unwrap();
        },
        None => todo!(),
    };

    match conf.section(Some("admin server")){
        Some(sec) => {
            config.admin_addr = sec.get("address").unwrap();
            config.admin_port = sec
                .get("port")
                .unwrap()
                .to_string()
                .parse::<u16>()?;
        },
        None => todo!(),
    }

    Ok(config)
}
