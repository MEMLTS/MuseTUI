use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeteaseCrypto{
    enc_text: String,
    enc_sec_key: String,
}

impl NeteaseCrypto{
    pub fn new(text: String)-> Self{
        crypto_netease(text)
    }
}

fn random_string(len: usize) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng();
    let mut result = String::new();

    for _ in 0..len {
        let random_index = rng.random_range(0..chars.len());
        let c = chars.chars().nth(random_index).unwrap();
        result.push(c);
    }
    result
}

fn crypto_netease(text: String) -> NeteaseCrypto {
    todo!()
}

mod test{
    #[test]
    fn test_range_string(){
        let result = crate::crypto::netease::random_string(16);
        println!("{:?}",result);
        assert_eq!(result.len(),16);
    }
}