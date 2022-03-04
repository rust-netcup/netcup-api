use serde_derive::{Deserialize, Serialize};
use std::env;
use tokio::{fs::File, io::AsyncReadExt};

use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub api_password: String,
    pub customer_number: u32,
}

impl Config {
    /// Tries to read the config file from all options.  
    /// We try in this order:
    /// 1. Check if variable names are set in the environment variables.  
    ///     ALL VARIABLES must be present in env for this to work.
    /// 2. Check if a config path is given in environment variables.
    ///     If so: Try loading the config at set path!
    /// 3. Try loading the config from the default path (current directory).
    ///
    /// Please note that this means that if all three env-vars are found in step 1, it will stop and use them. Thus, it's possible to override config variables via env-vars.
    pub async fn read() -> Result<Config, Error> {
        match Self::read_from_env().await {
            Ok(config) => Ok(config),
            Err(e) => {
                dbg!("{}", e);
                match Self::read_from_file_in_env().await {
                    Ok(config) => Ok(config),
                    Err(e) => {
                        dbg!("{}", e);
                        match Self::read_from_file("config.toml").await {
                            Ok(config) => Ok(config),
                            Err(e) => {
                                dbg!("{}", e);
                                Err(Error::from(
                                    "No configuration found or missing/invalid values inside configs!",
                                ))
                            }
                        }
                    }
                }
            }
        }
    }

    /// Tries to read variables from the environment.  
    /// All three must be present for this to work.  
    /// If not, an error will be returned.
    ///
    /// Env-Variables are:
    ///  - API_KEY
    ///  - API_SECRET
    ///  - CUSTOMER_NUMBER
    pub async fn read_from_env() -> Result<Config, Error> {
        let api_key = env::var("API_KEY").map_err(|e| e.to_string())?;
        dbg!("{}", &api_key);
        let api_password = env::var("API_SECRET").map_err(|e| e.to_string())?;
        dbg!("{}", &api_password);
        let customer_number = env::var("CUSTOMER_NUMBER").map_err(|e| e.to_string())?;
        dbg!("{}", &customer_number);

        Ok(Self {
            api_key,
            api_password,
            customer_number: customer_number.parse().unwrap(),
        })
    }

    /// Tries to read variables from a file, set as an env-variable.  
    /// Said variable is: CONFIG_PATH
    pub async fn read_from_file_in_env() -> Result<Config, Error> {
        let config_path = env::var("CONFIG_PATH").map_err(|e| e.to_string())?;
        dbg!("{}", &config_path);

        Self::read_from_file(&config_path).await
    }

    /// Tries to read variables from a file at given `path`.
    pub async fn read_from_file(path: &str) -> Result<Config, Error> {
        dbg!("{}", path);
        let mut file = File::open(path).await.map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await.map_err(|e| e.to_string())?;

        let content = String::from_utf8(buffer).map_err(|e| e.to_string())?;
        let config = toml::from_str(&content).map_err(|e| e.to_string())?;

        Ok(config)
    }
}
