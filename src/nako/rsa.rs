use rsa::{
    Result,
    Pkcs1v15Encrypt, 
    RsaPrivateKey, 
    RsaPublicKey,
    pkcs1::{
        LineEnding,
        DecodeRsaPrivateKey,
        DecodeRsaPublicKey,
        EncodeRsaPublicKey,
        EncodeRsaPrivateKey,
    },
};

/// 生成证书
pub fn generate_key() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();
    let bits = 2048;

    let priv_key = RsaPrivateKey::new(&mut rng, bits)?;
    let pub_key = RsaPublicKey::from(&priv_key);

    let priv_pem = priv_key.to_pkcs1_pem(LineEnding::LF)?;
    let pub_pem = pub_key.to_pkcs1_pem(LineEnding::LF)?;
    
    Ok((priv_pem.to_string(), pub_pem.to_string()))
}

pub fn encrypt(pubkey: &str, data: &[u8]) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();

    let pub_key = RsaPublicKey::from_pkcs1_pem(pubkey)?;
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])?;

    Ok(enc_data)
}

pub fn decrypt(privkey: &str, data: &[u8]) -> Result<Vec<u8>> {
    let priv_key = RsaPrivateKey::from_pkcs1_pem(privkey)?;

    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &data)?;

    Ok(dec_data)
}