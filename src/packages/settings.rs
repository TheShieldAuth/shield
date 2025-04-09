use config::{Config, ConfigError, Environment, File, Value};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::Deserialize;
use std::{env, fmt, path::Path, sync::Arc};

use crate::utils::helpers::default_cred::DefaultCred;

pub static SETTINGS: Lazy<Arc<RwLock<Settings>>> = Lazy::new(|| Arc::new(RwLock::new(Settings::new().expect("Failed to setup settings"))));

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
    // pub domain: String,
    pub host: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub uri: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Smtp {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Admin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Secrets {
    pub signing_key: String,
    pub api_key_signing_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    // pub environment: String,
    pub server: Server,
    pub logger: Logger,
    pub database: Database,
    pub smtp: Smtp,
    pub admin: Admin,
    pub secrets: Secrets,
    pub default_cred: DefaultCred,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| panic!("RUN_MODE environment variable not set"));

        let mut builder = Config::builder()
            .add_source(File::with_name("config/env/default"))
            .add_source(File::with_name(&format!("config/env/{}", run_mode)).required(false))
            // Add in settings from environment variables (with '.' as separator)
            // E.g. `server.port=5000` would set `server.port`
            .add_source(Environment::default().separator("."));

        // "./logs/default_cred.json" exists then read it else skip
        if Path::new("./logs/default_cred.json").exists() {
            let default_cred = DefaultCred::from_file().expect("Failed to read credentials");
            builder = builder.set_override("default_cred.realm_id", default_cred.realm_id.to_string())?;
            builder = builder.set_override("default_cred.client_id", default_cred.client_id.to_string())?;
            builder = builder.set_override("default_cred.master_admin_user_id", default_cred.master_admin_user_id.to_string())?;
            builder = builder.set_override("default_cred.master_api_key", default_cred.master_api_key.to_string())?;
            builder = builder.set_override("default_cred.resource_group_id", default_cred.resource_group_id.to_string())?;

            let resource_ids_value: Vec<Value> = default_cred.resource_ids.iter().map(|uuid| Value::new(None, uuid.to_string())).collect();
            builder = builder.set_override("default_cred.resource_ids", resource_ids_value)?;
        } else {
            builder = builder.set_override("default_cred.realm_id", "00000000-0000-0000-0000-000000000000")?;
            builder = builder.set_override("default_cred.client_id", "00000000-0000-0000-0000-000000000000")?;
            builder = builder.set_override("default_cred.master_admin_user_id", "00000000-0000-0000-0000-000000000000")?;
            builder = builder.set_override("default_cred.resource_group_id", "00000000-0000-0000-0000-000000000000")?;
            builder = builder.set_override("default_cred.master_api_key", "00000000-0000-0000-0000-000000000000")?;
            builder = builder.set_override("default_cred.resource_ids", vec!["00000000-0000-0000-0000-000000000000"])?;
        }

        builder.build()?.try_deserialize()
    }

    pub fn reload() -> Result<(), ConfigError> {
        let new_settings = Settings::new()?;
        let mut settings = SETTINGS.write();
        *settings = new_settings;
        Ok(())
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "http://localhost:{}", &self.port)
    }
}
