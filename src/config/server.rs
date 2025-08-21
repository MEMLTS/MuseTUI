use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct ServerConfig{
    cookie: Option<String>
}

impl ServerConfig{
    pub fn cookie(&self) -> &str{
        self.cookie.as_deref().unwrap_or( "")
    }
}