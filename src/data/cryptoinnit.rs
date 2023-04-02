extern crate crypto;

use crypto::aes::{ecb_decryptor, ecb_encryptor, KeySize};
use crypto::blockmodes::NoPadding;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use rand::{rngs::OsRng, Rng};

pub fn hash_string(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(s);
    hasher.result_str()
}

pub fn encrypt_string(key: &[u8;32], text: &str) -> Result<String, &'static str> {
    //let mut rand = OsRng;
    //rand.fill(&mut key);
    let mut encryptor = ecb_encryptor(KeySize::KeySize256, key, NoPadding);
    let mut ciphertext = Vec::<u8>::new();
    let mut buffer = [0; 4096];
    let new_text = "derpfacefuckfufkck";
    let bytes_text = new_text.as_bytes();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(bytes_text);
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor
            .encrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();
        ciphertext.extend(write_buffer.take_read_buffer().take_remaining());
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    let string_text = String::from_utf8(ciphertext).unwrap();
    Ok(string_text)
}

pub fn decrypt_string(mut key: [u8; 32], encryptedText: &str) -> String {
    let mut decryptor = ecb_decryptor(KeySize::KeySize256, &key, NoPadding);
    let mut decrypted = Vec::<u8>::new();
    let mut buffer = [0; 4096];
    let string_to_byte = encryptedText.as_bytes();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(string_to_byte);
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        let result = decryptor
            .decrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();
        decrypted.extend_from_slice(write_buffer.take_read_buffer().take_remaining());
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    let decrypted_text = std::str::from_utf8(&decrypted).unwrap();
    let as_string = decrypted_text.to_string();
    as_string
}
