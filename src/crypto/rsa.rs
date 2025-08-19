use rsa::{BigUint, RsaPublicKey, Pkcs1v15Encrypt};
use rand::rngs::ThreadRng;
use anyhow::{Result, Context};
use hex;

/// PKCS#1 v1.5 encryption
pub fn rsa_encrypt(data: &str, pub_exp_hex: &str, modulus_hex: &str) -> Result<String> {
    let e = BigUint::parse_bytes(pub_exp_hex.as_bytes(), 16)
        .context("Failed to parse public exponent (e) from hex")?;
    let n = BigUint::parse_bytes(modulus_hex.as_bytes(), 16)
        .context("Failed to parse modulus (n) from hex")?;

    let pub_key = RsaPublicKey::new(n, e)
        .context("Failed to create RSA public key")?;

    let data = data.as_bytes();

    let mut rng: ThreadRng = rand::thread_rng();
    let ciphertext = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, data)
        .context("Encryption failed")?;

    Ok(hex::encode(ciphertext))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rsa_enc(){
        let res = rsa_encrypt("Qym4o8nTOLeWv2Gl", "010001", "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76f61e785e89e1ae8e2c8fa5b13bc933f7b143e4b804da5b15db04d49d5c3a3c4f65e2e6095f415e3b0cd3a0d2d09eeb92b2d2b0ff").unwrap();
        println!("{:?}",res);
    }
}