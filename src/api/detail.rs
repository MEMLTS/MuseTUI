use crate::crypto::netease::NeteaseCrypto;
use crate::utils::request::RequestClient;

/// 这个接口需要csrf_token，暂时不能用

pub async fn get_detail(id: &str) -> anyhow::Result<serde_json::Value> {
   let url = "https://music.163.com/weapi/v3/song/detail?csrf_token=";
   let req = RequestClient::new();

   let json_string = format!(r#"{{"c":[{}],"ids":[{}], "csrf_token":""}}"#,id, id);

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
    async fn test_get_detail() {
        let result = super::get_detail("2014232695").await;
       println!("{:?}", result);
       assert!(result.is_ok())
    }
}