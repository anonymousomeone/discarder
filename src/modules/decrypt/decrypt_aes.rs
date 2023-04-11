use aes_gcm::{
    Aes256Gcm, 
    Nonce, 
    aead::{Aead, KeyInit}
};

pub fn decrypt(encrypted: &[u8], key: &[u8]) -> Vec<u8> {
    let iv = &encrypted[3..15];
    
    let payload = &encrypted[15..];

    let cipher = Aes256Gcm::new(key.into());

    let nonce = Nonce::from_slice(iv);

    cipher.decrypt(nonce, payload.as_ref()).unwrap()
}