use crate::utils::request::RequestClient;
use serde_json::json;
use crate::crypto::netease::NeteaseCrypto;

pub async fn get_lyric(id: &str) -> anyhow::Result<serde_json::Value> {
    let req = RequestClient::new();
    let url = "https://music.163.com/weapi/song/lyric";
    let json = json!({
        "id": id.parse::<i32>()?,
        "lv": -1,
        "tv": -1,
        "csrf_token": ""
    });

    let encrypted_json = NeteaseCrypto::new(&json.to_string())?;
    let result = req.post::<serde_json::Value>(
        &url,
        None,
        &encrypted_json
    ).await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_lyric() {
        let res = get_lyric("158135465").await;
        match res {
            Ok(data) => println!("{:#?}", data),
            Err(err) => eprintln!("Error: {:#?}", err),
        }
    }
}
