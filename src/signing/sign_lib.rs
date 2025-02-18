use rsa::{pkcs8::DecodePrivateKey, Pkcs1v15Sign, RsaPrivateKey};
use sha2::{Digest, Sha256};

use super::keys::PRIVATE_KEY;


pub fn sign_message<'a>(msg: &'a str) -> Vec<u8> {
    let priv_key = RsaPrivateKey::from_pkcs8_pem(PRIVATE_KEY).unwrap();
    
    // hash message
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(msg);
    let hash = hasher.finalize();

    priv_key.sign(Pkcs1v15Sign::new::<Sha256>(), &hash).unwrap()
}


#[cfg(test)]
mod tests {
    use super::sign_message;


    #[test]
    fn test_signing() {
        let msg = "My Message";
        let signature = sign_message(msg);

        // ToDo: verify
    }

}