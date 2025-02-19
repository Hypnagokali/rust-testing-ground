use rsa::{pkcs8::{DecodePrivateKey, DecodePublicKey}, Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey};
use sha2::{digest::{consts::U32, generic_array::{ArrayLength, GenericArray}}, Digest, Sha256};

use super::keys::{PRIVATE_KEY, PUBKEY};


pub fn hash_message(msg: &str) -> GenericArray<u8, U32> {
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(msg);
    hasher.finalize()    
}

pub fn sign_message(msg: &str) -> Vec<u8> {
    let priv_key = RsaPrivateKey::from_pkcs8_pem(PRIVATE_KEY).unwrap();
    
    let hash = hash_message(msg);

    priv_key.sign(Pkcs1v15Sign::new::<Sha256>(), &hash).unwrap()
}

pub fn verify_message(msg: &str, signature: &Vec<u8>) -> bool {
    let pub_key = RsaPublicKey::from_public_key_pem(PUBKEY).unwrap();
    
    let hash = hash_message(msg);
    
    match pub_key.verify(Pkcs1v15Sign::new::<Sha256>(), &hash, &signature) {
        Ok(_) => true,
        Err(_) => false,
    }
}


#[cfg(test)]
mod tests {
    use super::{sign_message, verify_message};


    #[test]
    fn test_signing() {
        let msg = "My Message";
        let signature = sign_message(msg);

        let is_authentic = verify_message(msg, &signature);

        assert!(is_authentic);
    }

}