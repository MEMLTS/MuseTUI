use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};
use anyhow::Result;
use crate::config::AppConfig;
use crate::utils::request::RequestClient;

pub async fn get_player(id: &str, level: Option<&str>) -> Result<serde_json::Value> {
    let url = "https://interface.music.163.com/api/song/enhance/player/url/v1";

    let mut form_data = HashMap::new();
    form_data.insert("ids", serde_json::json!([id]).to_string());
    form_data.insert("level", level.unwrap_or("jyeffect").to_string());
    form_data.insert("immerseType", "c51".to_string());
    form_data.insert("encodeType", "aac".to_string());
    form_data.insert("trialMode", "-1".to_string());
    form_data.insert("e_r", "true".to_string());

    let mut headers = HeaderMap::new();
    macro_rules! insert_hdr {
        ($h:expr, $k:expr, $v:expr) => {
            $h.insert($k, HeaderValue::from_static($v));
        };
    }
    insert_hdr!(headers, "User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:134.0) Gecko/20100101 Firefox/134.0");
    insert_hdr!(headers, "Origin", "orpheus://orpheus");
    insert_hdr!(headers, "Sec-Ch-Ua", "\"Chromium\";v=\"91\"");
    insert_hdr!(headers, "Sec-Ch-Ua-Mobile", "?0");
    insert_hdr!(headers, "Sec-Fetch-Site", "cross-site");
    insert_hdr!(headers, "Sec-Fetch-Mode", "cors");
    insert_hdr!(headers, "Sec-Fetch-Dest", "empty");
    insert_hdr!(headers, "Accept-Language", "en-US,en;q=0.9");

    let cookie = AppConfig::get().server().cookie();

    if !cookie.is_empty() {
        headers.insert("Cookie", HeaderValue::from_str(&cookie)?);
    }

    let req = RequestClient::new();
    let client = req.client();

    let response = client
        .post(url)
        .headers(headers)
        .form(&form_data)
        .send()
        .await?
        .error_for_status()?
        .json::<serde_json::Value>()
        .await?;

    Ok(response)
}