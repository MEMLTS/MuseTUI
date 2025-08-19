use crate::crypto::netease::NeteaseCrypto;
use crate::utils::request::RequestClient;

pub async fn get_search(query: &str) -> anyhow::Result<serde_json::Value>{
    let req = RequestClient::new();
    let url = "https://music.163.com/weapi/search/suggest/web?csrf_token=";
    let json_string = format!(r#"{{"s": "{}","limit": "8","csrf_token": ""}}"#, query);

    let encrypted_json = NeteaseCrypto::new(&json_string)?;
    let result = req.post::<serde_json::Value>(
        &url,
        None,
        &encrypted_json
    ).await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_get_search() {
        let result = super::get_search("创作者之死").await;
        println!("{:?}", result);
        assert!(result.is_ok())
    }
}