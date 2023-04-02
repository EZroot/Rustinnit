use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write, Read};

use crate::FILE_PATH_USERACCOUNT;

#[derive(Serialize, Deserialize)]
pub struct UserAccount {
    pub username: String,
    pub password: String,
    pub passsha: String,
    pub passusersha: String,
}

impl UserAccount {
    pub fn new(username: String, 
        password: String, 
        passsha: String, 
        passusersha: String) -> UserAccount {
        UserAccount { username, password, passsha, passusersha }
    }

    pub fn save_account(user: &UserAccount, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)
        .expect("Unable to create file");

        let json_string = serde_json::to_string(user)
        .expect("Failed to serialize useraccount");
        
        //Reference to bytes
        let json_bytes = json_string.as_bytes();

        //Write to the file
        file.write_all(json_bytes)
        .expect("Unable to write useraccount to file");

        print!("Json: {0}", &json_string);

        Ok(())
    }

    pub fn load_account(path:&str) -> UserAccount {

        let mut json_string = String::new();

        let mut file = File::open(path)
        .expect("Failed to load account");
        
        file.read_to_string(&mut json_string)
        .expect("Failed to read json string");

        let userAccount: UserAccount = serde_json::from_str(&json_string)
        .expect("Failed to serialize useraccount");

        //return user
        userAccount
    }

}