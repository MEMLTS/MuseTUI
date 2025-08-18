use reqwest::Client;
use reqwest::header::HeaderMap;
use serde::Deserialize;

pub struct Request{
    client: Client
}

impl Request{
    pub fn new()-> Request{
        Self {
            client: Client::new()
        }
    }
    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        header_map: Option<HeaderMap>
    ) -> Result<T,anyhow::Error> {
        let res = self
            .client
            .get(url)
            .headers(header_map.unwrap_or(HeaderMap::new()))
            .send()
            .await?;
        let text = res.text().await?;
        Ok(serde_json::from_str(&text)?)
    }

    pub async fn post<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        header_map: Option<HeaderMap>,
        json: &serde_json::Value,
    ) -> Result<T,anyhow::Error> {
        let res = self
            .client
            .post(url)
            .json(json)
            .headers(header_map.unwrap_or(HeaderMap::new()))
            .send()
            .await?;
        let text = res.text().await?;
        Ok(serde_json::from_str(&text)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get() {
        let request = Request::new();
        let res = request.get::<serde_json::Value>("https://jsonplaceholder.typicode.com/posts/1", None).await;
        println!("{:#?}", res.unwrap());
    }
}