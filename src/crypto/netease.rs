use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::crypto::aes::aes_encrypt_cbc;
use crate::crypto::rsa::rsa_encrypt;
use base64::{engine::general_purpose::STANDARD, Engine};

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeteaseCrypto{
    enc_text: String,
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
    let res_result = rsa_encrypt(
        &random_string,
        "010001",
        "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7"
    )?;

    let et = STANDARD.encode(hex::decode(second)?);

    Ok(NeteaseCrypto{
        enc_text: String::try_from(Vec::from(et))?,
        enc_sec_key: res_result,
    })
}

mod test{
    #[test]
    fn test_range_string(){
        let result = crate::crypto::netease::random_string(16);
        println!("{:?}",result);
        assert_eq!(result.len(),16);
    }
    #[test]
    fn test_crypto_netease(){
        let result = crate::crypto::netease::crypto_netease("hello world");
        println!("{:#?}",result.unwrap());
    }
}