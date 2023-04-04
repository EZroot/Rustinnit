mod data {
    pub mod cryptoinnit;
    pub mod networkinnit;
    pub mod useraccount;
}

use rand::{rngs::OsRng, Rng};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};

use crate::data::{cryptoinnit, networkinnit, useraccount::UserAccount};

const FILE_PATH_CLIENTSTATE: &str = "clientstate.json";
const FILE_PATH_USERACCOUNT: &str = "useraccount.json";

fn main() {
    let mut user_account = UserAccount::load_account(FILE_PATH_USERACCOUNT);
    loop {
        println!("1) New User");
        println!("2) Load User");
        println!("3) Save User");
        println!("4) Quit");
        print!("{0}@{1}>:",user_account.username, networkinnit::get_public_ip().unwrap());

        std::io::stdout().flush().expect("Failed to flush console!");

        let mut choice_input = String::new();
     
        std::io::stdin()
            .read_line(&mut choice_input)
            .expect("Failed to read line for choice");

        match choice_input.trim() {
            "1" => user_account.setup_user(FILE_PATH_USERACCOUNT),
            "2" => user_account = UserAccount::load_account(FILE_PATH_USERACCOUNT),
            "3" => user_account.save_account(FILE_PATH_USERACCOUNT).unwrap(),
            "4" => break,
            _ => user_account.setup_user(FILE_PATH_USERACCOUNT),
        }
    }
}
