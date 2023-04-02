mod data {
    pub mod cryptoinnit;
    pub mod useraccount;
}

use data::useraccount::UserAccount;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use rand::{rngs::OsRng, Rng};

use crate::data::cryptoinnit;

const FILE_PATH_CLIENTSTATE: &str = "clientstate.json";
const FILE_PATH_USERACCOUNT: &str = "useraccount.json";

fn main() {
    let username = "Willis";
    let password = "NerdFace";
    let pass_hash = cryptoinnit::hash_string(password);
    let pass_user = username.to_owned() + password;
    let pass_user_hash = cryptoinnit::hash_string(&pass_user);

    let new_user = UserAccount::new(username.to_owned(), password.to_owned(), pass_hash, pass_user_hash);

    // print!(
    //     "User {0} {1} {2} {3}",
    //     new_user.username, new_user.password, new_user.passsha, new_user.passusersha
    // );

    UserAccount::save_account(&new_user, FILE_PATH_USERACCOUNT).expect("Failed to save account");

    let loaded_user = UserAccount::load_account(FILE_PATH_USERACCOUNT);

    // print!(
    //     "We loaded {0} {1} {2} {3} ",
    //     loaded_user.username, loaded_user.password, loaded_user.passsha, loaded_user.passusersha
    // );

    print!("---------------------------------------------------------------------------------");
    print!("Ok now lets encrypt this shit");
    print!("---------------------------------------------------------------------------------");
    let information = loaded_user.username + &loaded_user.password;
    let mut rand = OsRng;
    let mut key = b"th231iss5551212222houldbe32chars";
    //rand.fill(&mut key);

    print!("KeyLen {0}", key.len());
    let information_encrypted = cryptoinnit::encrypt_string(&key, &information).unwrap();

    //let key_to_string = String::from_utf8(key.to_vec()).expect("Couldn't convert the u8 key to string");
    print!("Encrypting Key: {0} \n[Encrypting Text]\n {1} \n[Encrypted Text]\n {2}", 
    "foo",
    &information,
    &information_encrypted);
    print!("---------------------------------------------------------------------------------");

}
