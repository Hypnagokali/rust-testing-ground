use aes::Aes256;
use base64::{prelude::BASE64_STANDARD, Engine};
use cbc::Encryptor;
use cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};

pub fn encrypt_aes256_cbc(msg: &str, key: &[u8; 32], iv: &[u8; 16]) -> String {
    let cipher = Encryptor::<Aes256>::new_from_slices(key, iv).unwrap();
    let res = cipher.encrypt_padded_vec_mut::<Pkcs7>(&mut msg.as_bytes());

    BASE64_STANDARD.encode(res)
}

#[cfg(test)]
mod tests {
    use super::encrypt_aes256_cbc;

    #[test]
    fn simple_test() {
        let key = [87u8; 32];
        let key2 = b"abcdefghijklmnopqrstuvwxyz123456";
        let iv = [71u8; 16];
        let res = encrypt_aes256_cbc("Hello, just a test", &key, &iv);
        let res2 = encrypt_aes256_cbc("Hello, just a test", &key2, &iv);

        println!("{res}");
        println!("{res2}");
    }
}