use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{Write, Read};
use std::path::Path;

use super::cryptoinnit;

#[derive(Serialize, Deserialize)]
pub struct UserAccount {
    pub username: String,
    pub password: String,
}

impl UserAccount {
    pub fn new(username: String, 
        password: String, 
        passsha: String, 
        passusersha: String) -> UserAccount {
        UserAccount { username, password }
    }

    pub fn encrypt_password(&mut self)
    {
        self.password = cryptoinnit::hash_string(&self.password);
    }

    pub fn setup_user(&mut self, path_to_save: &str)
    {
        let mut username = String::new();
        let mut password = String::new();

        println!("Enter a Username: ");
        std::io::stdout().flush().expect("Failed to flush console!");

        std::io::stdin().read_line(&mut username).expect("Failed to read line for username");

        println!("Now, enter a Password: ");
        std::io::stdout().flush().expect("Failed to flush console!");

        std::io::stdin().read_line(&mut password).expect("Failed to read line for password");
        
        self.username = username.trim().to_string();
        self.password = password.trim().to_string();
        self.encrypt_password();
        self.save_account(path_to_save).unwrap();
    }

    pub fn to_json(&self) -> String {
        let json_string = serde_json::to_string(self).expect("Failed to convert self to json string");
        json_string
    }

    pub fn save_account(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)
        .expect("Unable to create file");

        let json_string = serde_json::to_string(self)
        .expect("Failed to serialize useraccount");
        
        //Reference to bytes
        let json_bytes = json_string.as_bytes();

        //Write to the file
        file.write_all(json_bytes)
        .expect("Unable to write useraccount to file");
        Ok(())
    }

    pub fn load_account(path:&str) -> UserAccount {

        let mut json_string = String::new();
        let mut user_account : UserAccount = UserAccount::new(String::new(), String::new(),String::new(),String::new());

        let file_path = Path::new(path);
        if file_path.exists()
        {
            let mut file = File::open(path)
            .expect("Failed to load account");
            
            file.read_to_string(&mut json_string)
            .expect("Failed to read json string");

            user_account = serde_json::from_str(&json_string)
            .expect("Failed to serialize useraccount");
        }
        user_account
    }

}