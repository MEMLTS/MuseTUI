use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct ServerConfig{
    cookie: String
}

impl ServerConfig{
    pub fn cookie(&self) -> &str{
        &self.cookie
    }
}