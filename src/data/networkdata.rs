use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{Write, Read};
use std::path::Path;

use crate::utils::filesinnit;
use crate::utils::filesinnit::load_data_from_file;

#[derive(Serialize, Deserialize)]
pub struct NetworkData {
    pub local_ip: String,
    pub public_ip: String
}

impl NetworkData {
    pub fn new(local_ip : String, public_ip:String) -> NetworkData{
        NetworkData {local_ip, public_ip}
    }

    pub fn save_data(&self, path : &str) -> std::io::Result<()>
    {
        filesinnit::save_data_to_file(self,path).unwrap();
        Ok(())
    }

    pub fn load_data(path:&str) -> NetworkData {
        let network_data : NetworkData = load_data_from_file::<NetworkData>(path).unwrap();
        network_data
    }
}

#[derive(Serialize, Deserialize)]
pub struct NetworkDataKeys {
    pub local_ip_key: String,
    pub public_ip_key: String
}

impl NetworkDataKeys {
    pub fn new(local_ip_key : String, public_ip_key:String) -> NetworkDataKeys{
        NetworkDataKeys {local_ip_key, public_ip_key}
    }

    pub fn save_data(&self, path : &str) -> std::io::Result<()>
    {
        filesinnit::save_data_to_file(self,path).unwrap();
        Ok(())
    }

    pub fn load_data(path:&str) -> NetworkDataKeys {
        let network_data : NetworkDataKeys = load_data_from_file::<NetworkDataKeys>(path).unwrap();
        network_data
    }
}