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

    // Calculate chunk size - JavaScript uses chunkSize = 2 * biHighIndex(this.m)
    // biHighIndex returns the index of the highest non-zero digit
    let modulus_bits = n.bits();
    let chunk_size = ((modulus_bits + 15) / 16) * 2; // Match JavaScript's calculation
    let chunk_size = chunk_size as usize;

    // Convert string to character codes (JavaScript: c[e] = b.charCodeAt(e))
    let mut char_codes = Vec::new();
    for ch in data.chars() {
        char_codes.push(ch as u32);
    }

    // Pad with zeros to chunk boundary (JavaScript: while (0 != c.length % a.chunkSize) c[e++] = 0)
    let data_len = char_codes.len();
    while char_codes.len() % chunk_size != 0 {
        char_codes.push(0);
    }

    let mut result_parts = Vec::new();

    // Process each chunk
    for chunk in char_codes.chunks(chunk_size) {
        // Convert chunk to BigUint following JavaScript's method
        // JavaScript: j.digits[h] = c[i++], j.digits[h] += c[i++] << 8
        let mut big_int_bytes = Vec::new();

        for i in (0..chunk.len()).step_by(2) {
            let low = chunk.get(i).unwrap_or(&0);
            let high = chunk.get(i + 1).unwrap_or(&0);
            let combined = low + (high << 8);

            // Convert to little-endian bytes for the BigUint
            big_int_bytes.push((combined & 0xFF) as u8);
            big_int_bytes.push(((combined >> 8) & 0xFF) as u8);
        }

        // Create BigUint from little-endian bytes
        let m = BigUint::from_bytes_le(&big_int_bytes);

        // Perform modular exponentiation: m^e mod n
        let encrypted = m.modpow(&e, &n);

        // Convert to hex string (JavaScript uses biToHex)
        let hex_str = format!("{:x}", encrypted);
        result_parts.push(hex_str);
    }

    // Join with spaces and trim final space (JavaScript: g.substring(0, g.length - 1))
    let result = result_parts.join(" ");
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rsa_enc(){
        // Use the complete correct modulus from the JavaScript reference
        let res = rsa_encrypt(
            "Qym4o8nTOLeWv2Gl",
            "010001",
            "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7"
        ).unwrap();
        println!("RSA encrypted (PKCS1v15): {}", res);

        // Test with the same parameters as the JavaScript version
        let test_key = "abcdef1234567890"; // 16 char test key
        let res2 = rsa_encrypt(
            test_key,
            "010001",
            "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7"
        ).unwrap();
        println!("RSA encrypted test key (PKCS1v15): {}", res2);
    }

    #[test]
    fn test_rsa_enc_raw(){
        // Test the raw implementation that matches JavaScript behavior
        let test_key = "abcdef1234567890"; // 16 char test key
        let res = rsa_encrypt_raw(
            test_key,
            "010001",
            "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7"
        ).unwrap();
        println!("RSA encrypted raw (JS compatible): {}", res);

        // Test with shorter string
        let short_key = "test1234";
        let res2 = rsa_encrypt_raw(
            short_key,
            "010001",
            "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7"
        ).unwrap();
        println!("RSA encrypted short key (JS compatible): {}", res2);
    }
}