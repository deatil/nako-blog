use rsa::{
    Result, 
    Pkcs1v15Encrypt, 
    pkcs8::LineEnding,
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
    RsaPrivateKey, 
    RsaPublicKey,
};

// 生成证书
pub fn generate_key(bits: usize) -> Result<(String, String)> {
    let mut rng = rand::thread_rng();

    let priv_key = RsaPrivateKey::new(&mut rng, bits)?;
    let pub_key = RsaPublicKey::from(&priv_key);

    let priv_pem = priv_key.to_pkcs8_pem(LineEnding::LF)?;
    let pub_pem = match pub_key.to_public_key_pem(LineEnding::LF) {
        Ok(v) => v,
        Err(_) => "".into(),
    };
    
    Ok((priv_pem.to_string(), pub_pem.to_string()))
}

// 加密
pub fn encrypt(pubkey: &str, data: &[u8]) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();

    if let Ok(pub_key) = RsaPublicKey::from_public_key_pem(pubkey) {
        let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])?;
    
        return Ok(enc_data);
    };

    Ok("".into())
}

// 解密
pub fn decrypt(privkey: &str, data: &[u8]) -> Result<Vec<u8>> {
    let priv_key = RsaPrivateKey::from_pkcs8_pem(privkey)?;

    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &data)?;

    Ok(dec_data)
}