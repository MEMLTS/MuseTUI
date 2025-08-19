use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::crypto::aes::aes_encrypt_cbc;
use crate::crypto::rsa::{rsa_encrypt, rsa_encrypt_raw};

#[derive(Debug,Serialize,Deserialize)]
pub struct NeteaseCrypto{
    #[serde(rename="params")]
    enc_text: String,
    #[serde(rename="encSecKey")]
    enc_sec_key: String,
}

impl NeteaseCrypto{
    pub fn new(text: &str)-> anyhow::Result<serde_json::Value>{
        Ok(
            serde_json::json!(crypto_netease(text)?)
        )
    }
}

/// 生成指定长度随机字符串
fn random_string(len: usize) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    (0..len).map(
        |_|{
            chars.chars().nth(rand::thread_rng().gen_range(0..chars.len())).unwrap()
        }
    ).collect()
}

fn crypto_netease(text: &str) -> anyhow::Result<NeteaseCrypto>{
    let aes_sec_key = "0CoJUm6Qyw8W8jud";
    let aes_iv = "0102030405060708";
    let first = aes_encrypt_cbc(
        text,
        aes_sec_key,
        aes_iv
    )?;
    let random_string = random_string(16);
    let second = aes_encrypt_cbc(
        &first,
        &random_string,
        aes_iv
    )?;

    let res_result = rsa_encrypt_raw(
        &random_string,
        "010001",
        "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7"
    )?;

    Ok(NeteaseCrypto{
        enc_text: second,
        enc_sec_key: res_result,
    })
}

mod test{
    use crate::crypto::netease::NeteaseCrypto;

    #[test]
    fn test_range_string(){
        let result = crate::crypto::netease::random_string(16);
        println!("{:#?}",result);
        assert_eq!(result.len(),16);
    }
    #[test]
    fn test_crypto_netease(){
        let result = crate::crypto::netease::crypto_netease("hello world");
        println!("{:#?}",result.unwrap());
    }

    #[test]
    fn test_json_ordering_consistency(){
        // Test that the same JSON content produces the same encrypted result
        let json1 = r#"{"id":2014232695,"lv":-1,"tv":-1,"csrf_token":""}"#;
        let json2 = r#"{"id":2014232695,"lv":-1,"tv":-1,"csrf_token":""}"#;

        let result1 = crate::crypto::netease::crypto_netease(json1).unwrap();
        let result2 = crate::crypto::netease::crypto_netease(json2).unwrap();

        // Since the random string is different each time, we can't compare directly
        // But we can ensure the function doesn't panic and produces results
        assert!(!result1.enc_text.is_empty());
        assert!(!result1.enc_sec_key.is_empty());
        assert!(!result2.enc_text.is_empty());
        assert!(!result2.enc_sec_key.is_empty());

        println!("JSON1 encryption: {:#?}", result1);
        println!("JSON2 encryption: {:#?}", result2);
    }

    #[test]
    fn test_complete_encryption_flow(){
        // Test the complete encryption flow with a known JSON
        let test_json = r#"{"id":123456,"lv":-1,"tv":-1,"csrf_token":""}"#;

        let result = crate::crypto::netease::crypto_netease(test_json).unwrap();

        // Verify structure
        assert!(!result.enc_text.is_empty());
        assert!(!result.enc_sec_key.is_empty());

        // Check that enc_text is base64 (should not contain spaces)
        assert!(!result.enc_text.contains(' '));

        // Check that enc_sec_key looks like hex (may contain spaces based on JavaScript implementation)
        // The RSA result should be valid hex characters
        let cleaned_sec_key = result.enc_sec_key.replace(' ', "");
        assert!(cleaned_sec_key.chars().all(|c| c.is_ascii_hexdigit()));

        println!("Complete encryption test:");
        println!("Input JSON: {}", test_json);
        println!("Encrypted text: {}", result.enc_text);
        println!("Encrypted sec key: {}", result.enc_sec_key);

        // Test NeteaseCrypto::new wrapper
        let json_result = NeteaseCrypto::new(test_json).unwrap();
        println!("NeteaseCrypto::new result: {:#?}", json_result);
    }
}