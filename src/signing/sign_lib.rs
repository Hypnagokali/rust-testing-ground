use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, EncodingKey, Header};
use rsa::{pkcs8::{DecodePrivateKey, DecodePublicKey}, Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey};
use sha2::{digest::{consts::U32, generic_array::GenericArray}, Digest, Sha256};

use super::{jwt::Claims, keys::{PRIVATE_KEY, PUBKEY}};


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

pub fn get_token() -> String {
    let iat = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

    let exp = iat + (60 * 5);

    let claims = Claims {
        sub: "hans.hirschhausen@example.org".to_owned(),
        name: "Hans Hirschhausen".to_owned(),
        iat,
        exp,
    };

    encode(
        &Header::new(jsonwebtoken::Algorithm::RS256), 
        &claims, 
        &EncodingKey::from_rsa_pem(PRIVATE_KEY.as_bytes()).unwrap()
    ).unwrap()
}

#[cfg(test)]
mod tests {
    use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use rsa::{pkcs8::DecodePublicKey, traits::PublicKeyParts, RsaPublicKey};
    use crate::signing::{jwt::{Claims, JWKS}, keys::PUBKEY, sign_lib::get_token};

    use super::{sign_message, verify_message};

    #[test]
    fn test_signing() {
        let msg = "My Message";
        let signature = sign_message(msg);

        let is_authentic = verify_message(msg, &signature);

        assert!(is_authentic);
    }

    #[test]
    fn extract_e_n() {
        // this is just for extracting the modulus and exponent from the key
        let pk = RsaPublicKey::from_public_key_pem(PUBKEY).unwrap();

        let e = pk.e().to_bytes_be();
        let n = pk.n().to_bytes_be();

        let eb64 = BASE64_URL_SAFE_NO_PAD.encode(e);
        let nb64 = BASE64_URL_SAFE_NO_PAD.encode(n);
        println!("e: {}, n: {}", eb64, nb64);
    }

    #[test]
    fn test_verify_token() {
        let jwt = get_token();
        let jwks = JWKS::test_without_cert_chain();

        let decoding_key = DecodingKey::from_rsa_components(&jwks.n, &jwks.e).unwrap();
        let validation = Validation::new(Algorithm::RS256);
        let decoded = decode::<Claims>(&jwt, &decoding_key, &validation).unwrap();

        assert_eq!(decoded.claims.name, "Hans Hirschhausen");
    }

}