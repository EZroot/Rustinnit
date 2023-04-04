use serde::{Deserialize,de::DeserializeOwned, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn save_text_to_file(text: &str, path : &str) -> std::io::Result<()> {
    let mut file = File::create(path).expect("Unable to create file");

    let file_bytes = text.as_bytes();

    file.write_all(file_bytes).expect("Failed to write obj to json");
    Ok(())
}

pub fn save_data_to_file<T>(obj: &T, path : &str) -> std::io::Result<()>
where
    T: Serialize,
{
    let mut file = File::create(path).expect("Unable to create file");

    let json_string = serde_json::to_string(obj).expect("Failed to serialize useraccount");

    let json_bytes = json_string.as_bytes();

    file.write_all(json_bytes).expect("Failed to write obj to json");
    Ok(())
}

pub fn load_text_from_file(path:&str) -> std::io::Result<String> {
    let mut file_text = String::new();
    let file_path = Path::new(path);
    if file_path.exists()
    {
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut file_text).unwrap();
    }
    Ok(file_text)
}

pub fn load_data_from_file<T: DeserializeOwned>(path:&str) -> std::io::Result<T> {
    let mut file_text = String::new();
    let file_path = Path::new(path);
    if file_path.exists()
    {
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut file_text).unwrap();
    }
    let obj:T = serde_json::from_str(&file_text).unwrap();
    Ok(obj)
}