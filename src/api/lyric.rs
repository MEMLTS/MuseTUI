use serde::Deserialize;
use crate::utils::request::RequestClient;
use crate::crypto::netease::NeteaseCrypto;

#[derive(Debug,Deserialize)]
#[allow(dead_code)]
pub struct Lrc{
    pub version: i32,
    pub lyric: String
}

#[derive(Debug,Deserialize)]
#[allow(dead_code)]
pub struct NeteaseLyric{
    pub code: i32,
    pub lrc: Lrc,
    pub qfy: bool,
    pub sfy: bool,
    pub sgc: bool,
    pub tlyric: Lrc,
}

pub async fn get_lyric(id: &str) -> anyhow::Result<NeteaseLyric> {
    let req = RequestClient::new();
    let url = "https://music.163.com/weapi/song/lyric";
    let json_string = format!(
        r#"{{"id":{},"lv":-1,"tv":-1,"csrf_token":""}}"#,
        id.parse::<i32>()?
    );

    let encrypted_json = NeteaseCrypto::new(&json_string)?;
    let result = req.post::<serde_json::Value>(
        &url,
        None,
        &encrypted_json
    ).await?;
    let lyric_data: NeteaseLyric = serde_json::from_value(result)?;

    Ok(lyric_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_lyric() {
        let res = get_lyric("2014232695").await;
        match res {
            Ok(data) => println!("{:#?}", data),
            Err(err) => eprintln!("Error: {:#?}", err),
        }
    }
}
