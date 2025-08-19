use aes::cipher::{BlockModeEncrypt, KeyIvInit};
use aes::Aes128;
use aes::cipher::block_padding::Pkcs7;
use anyhow::anyhow;
use cbc::Encryptor;
use base64::{engine::general_purpose::STANDARD, Engine};

type Aes128CbcEnc = Encryptor<Aes128>;

pub fn aes_encrypt_cbc(text: &str, key: &str, iv: &str) -> anyhow::Result<String> {
    let key_bytes = key.as_bytes();
    let iv_bytes = iv.as_bytes();

    let key_array: [u8; 16] = key_bytes.try_into()
        .map_err(|_| anyhow!("Key must be 16 bytes long"))?;
    let iv_array: [u8; 16] = iv_bytes.try_into()
        .map_err(|_| anyhow!("IV must be 16 bytes long"))?;

    let cipher = Aes128CbcEnc::new(key_array.as_ref(), iv_array.as_ref());

    let text_bytes = text.as_bytes();

    let mut buffer = vec![0u8; text_bytes.len() + 16];

    let encrypted_data = cipher.encrypt_padded_b2b::<Pkcs7>(
        &text_bytes,
        &mut buffer
    ).map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;

    Ok(STANDARD.encode(encrypted_data))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_aes_encrypt_cbc() {
        let text = "hello world";
        let key = "xzxdnslcWZwQkoDx";
        let iv = "0102030405060708";
        let result = aes_encrypt_cbc(text, key, iv).unwrap();
        assert_eq!(result, "40dc795fbdaad35c1ff6d73892501a43");
    }
}