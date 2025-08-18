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

    (0..len).map(
        |_|{
            chars.chars().nth(rand::thread_rng().gen_range(0..chars.len())).unwrap()
        }
    ).collect()
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