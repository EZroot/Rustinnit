mod data {
    pub mod useraccount;
    pub mod networkdata;
}

mod utils{
    pub mod cryptoinnit;
    pub mod filesinnit;
    pub mod networkinnit;
}

use std::io::{Write};

use data::networkdata::NetworkDataKeys;
use utils::filesinnit;

use crate::data::{useraccount::UserAccount, networkdata::NetworkData};
use crate::utils::{cryptoinnit, networkinnit};

const FILE_PATH_CLIENTSTATE: &str = "clientstate.json";
const FILE_PATH_USERACCOUNT: &str = "useraccount.json";
const FILE_PATH_NETWORKDATA: &str = "networkdata.json";
const FILE_PATH_IP_KEYS: &str = "ipkeys.json";

fn main() {
    //load user
    let mut user_account = UserAccount::load_account(FILE_PATH_USERACCOUNT);

    //ip initialization
    let public_ip = networkinnit::get_public_ip().unwrap();
    let private_ip = networkinnit::get_local_ip();
    
    let private_ip_encrypted = cryptoinnit::encrypt_string(&private_ip).unwrap();
    let public_ip_encrypted = cryptoinnit::encrypt_string(&public_ip).unwrap();

    //save network data + ip keys
    let network_data = NetworkData::new(private_ip_encrypted.0,public_ip_encrypted.0);
    let network_data_keys = NetworkDataKeys::new(private_ip_encrypted.1, public_ip_encrypted.1);

    network_data.save_data(FILE_PATH_NETWORKDATA).unwrap();
    network_data_keys.save_data(FILE_PATH_IP_KEYS).unwrap();

    //program loop
    loop {
        //crypto test
        let crypto_test = cryptoinnit::encrypt_string("dppp").unwrap();
        cryptoinnit::decrypt_string(&crypto_test.0, &crypto_test.1).unwrap();

        //options
        println!("1) New User");
        println!("2) Load User");
        println!("3) Save User");
        println!("4) Quit");
        print!("{0}@{1}>:",user_account.username, &public_ip);
        //force draw text
        std::io::stdout().flush().expect("Failed to flush console!");

        //wait for text
        let mut choice_input = String::new();
        std::io::stdin().read_line(&mut choice_input).expect("Failed to read line for choice");
        
        //decide
        match choice_input.trim() {
            "1" => user_account.setup_user(FILE_PATH_USERACCOUNT),
            "2" => user_account = UserAccount::load_account(FILE_PATH_USERACCOUNT),
            "3" => user_account.save_account(FILE_PATH_USERACCOUNT).unwrap(),
            "4" => break,
            _ => user_account.setup_user(FILE_PATH_USERACCOUNT),
        }
    }
}
