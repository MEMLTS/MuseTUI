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

/// Custom RSA encryption matching JavaScript implementation exactly
pub fn rsa_encrypt_raw(data: &str, pub_exp_hex: &str, modulus_hex: &str) -> Result<String> {
    let e = BigUint::parse_bytes(pub_exp_hex.as_bytes(), 16)
        .context("Failed to parse public exponent (e) from hex")?;
    let n = BigUint::parse_bytes(modulus_hex.as_bytes(), 16)
        .context("Failed to parse modulus (n) from hex")?;

    let modulus_bits = n.bits();
    let chunk_size = ((modulus_bits + 15) / 16) * 2;
    let chunk_size = chunk_size;

    let mut char_codes = Vec::new();
    for ch in data.chars() {
        char_codes.push(ch as u32);
    }

    while char_codes.len() % chunk_size != 0 {
        char_codes.push(0);
    }

    let mut result_parts = Vec::new();

    for chunk in char_codes.chunks(chunk_size) {
        let mut big_int_bytes = Vec::new();

        for i in (0..chunk.len()).step_by(2) {
            let low = chunk.get(i).unwrap_or(&0);
            let high = chunk.get(i + 1).unwrap_or(&0);
            let combined = low + (high << 8);

            big_int_bytes.push((combined & 0xFF_u32) as u8);
            big_int_bytes.push(((combined >> 8) & 0xFF_u32) as u8);
        }

        let m = BigUint::from_bytes_le(&big_int_bytes);

        let encrypted = m.modpow(&e, &n);

        let hex_str = format!("{:x}", encrypted);
        result_parts.push(hex_str);
    }

    let result = result_parts.join(" ");
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rsa_enc(){
        let res = rsa_encrypt(
            "Qym4o8nTOLeWv2Gl",
            "010001",
            "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7"
        ).unwrap();
        println!("RSA encrypted (PKCS1v15): {}", res);
    }
}