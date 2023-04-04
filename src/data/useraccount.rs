use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{Write, Read};
use std::path::Path;

use crate::utils::filesinnit::load_data_from_file;
use crate::utils::{cryptoinnit, filesinnit};


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

    pub fn save_account(&self, path: &str) -> std::io::Result<()> {
       filesinnit::save_data_to_file(self,path).unwrap();

        Ok(())
    }

    pub fn load_account(path:&str) -> UserAccount {
        let user_account : UserAccount = load_data_from_file::<UserAccount>(path).unwrap();
        user_account
    }

}