extern crate crypto;
use aes_gcm::{
    aead::{Aead,generic_array::GenericArray, KeyInit, OsRng},
    Aes256Gcm,
    Nonce, // Or `Aes128Gcm`
};

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use hex;
use arrayvec::ArrayVec;

const NONCE_BYTE_LENGTH : usize = 12;

pub fn hash_string(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(s);
    hasher.result_str()
}

///```
/// Returns (CipherText, Key)
///```
pub fn encrypt_string(message : &str) -> Result<(String, String), aes_gcm::aead::Error> {
    //ideally we also seperate the nonce, but for this, lets just include it
    //generate a key of bytes using aes way of generating
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);

    //randomly fill the nonce array with bytes
    let mut nonce : Vec<u8> = vec![0; NONCE_BYTE_LENGTH];
    getrandom::getrandom(&mut nonce).unwrap();

    //convert to generic array for encyrption use
    let nonce_generic = GenericArray::from_slice(&nonce);

    //encrypt out text
    let mut ciphertext = cipher.encrypt(nonce_generic, message.as_ref()).unwrap().to_vec();
    //include our nonce
    ciphertext.extend(&nonce);
    
    //make the bytes readable as a string through hex
    let cipher_hex_str = hex::encode(&ciphertext);
    let key_hex_str = hex::encode(&key);
    println!("Cipher hex: {}", cipher_hex_str);
    println!("Key hex: {}", key_hex_str);

    Ok((cipher_hex_str, key_hex_str))
}

pub fn decrypt_string(message : &str, key : &str) -> Result<String, aes_gcm::aead::Error> {

    let decoded_key = hex::decode(key).unwrap();
    let mut decoded_message = hex::decode(message).unwrap();

    let nonce = decoded_message.split_off(decoded_message.len() - NONCE_BYTE_LENGTH);

    let nonce_bytes = GenericArray::from_slice(&nonce);
    let key_bytes = GenericArray::from_slice(&decoded_key);

    let cipher = Aes256Gcm::new(&key_bytes);

    let decrypted_text = cipher.decrypt(nonce_bytes, decoded_message.as_ref()).unwrap();

    let plain_text = String::from_utf8(decrypted_text).expect("Couldnt encode to UTF8... bytes arent ASCII");

    println!("Plain text: {}", plain_text);
    Ok(plain_text)
}