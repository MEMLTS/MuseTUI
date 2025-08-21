use std::fs;
use std::sync::LazyLock;
use anyhow::Context;
use config::Config;
use serde::Deserialize;
use crate::config::server::ServerConfig;

mod server;

static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load().expect("Failed to load config"));

#[derive(Debug,Deserialize)]
pub struct AppConfig{
    server: ServerConfig
}

impl AppConfig{
    pub fn load() -> anyhow::Result<Self>{
        if !std::path::Path::new("config.yaml").exists(){
            fs::write("config.yaml", include_str!("def_config.yaml"))?;
        }
        Config::builder()
            .add_source(
                config::File::with_name("config.yaml")
                    .format(config::FileFormat::Yaml)
                    .required(true)
            )
            .build()?
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Failed to load config"))
    }
    pub fn get() -> &'static AppConfig{
        &CONFIG
    }
    pub fn server(&self) -> &ServerConfig{
        &self.server
    }
}