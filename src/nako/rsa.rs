use rsa::{
    Result, 
    Pkcs1v15Encrypt, 
    pkcs8::LineEnding,
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
    RsaPrivateKey, 
    RsaPublicKey,
};

/// 生成证书
pub fn generate_key() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();
    let bits = 1024;

    let priv_key = RsaPrivateKey::new(&mut rng, bits)?;
    let pub_key = RsaPublicKey::from(&priv_key);

    let priv_pem = priv_key.to_pkcs8_pem(LineEnding::LF)?;
    let pub_pem = match pub_key.to_public_key_pem(LineEnding::LF) {
        Ok(v) => v,
        Err(_) => "".into(),
    };
    
    Ok((priv_pem.to_string(), pub_pem.to_string()))
}

pub fn encrypt(pubkey: &str, data: String) -> Result<String> {
    let mut rng = rand::thread_rng();

    let data = data.as_bytes();

    if let Ok(pub_key) = RsaPublicKey::from_public_key_pem(pubkey) {
        let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])?;

        let enc_data = match String::from_utf8(enc_data) {
            Ok(v) => v,
            Err(_) => "".into(),
        };
    
        return Ok(enc_data);
    };

    Ok("".to_string())
}

pub fn decrypt(privkey: &str, data: String) -> Result<String> {
    let priv_key = RsaPrivateKey::from_pkcs8_pem(privkey)?;

    let data = data.as_bytes();

    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &data)?;

    let dec_data = match String::from_utf8(dec_data) {
        Ok(v) => v,
        Err(e) => format!("{}", e),
    };

    Ok(dec_data)
}