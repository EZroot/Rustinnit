use if_addrs::get_if_addrs;
use std::net::{IpAddr, Ipv4Addr};
use reqwest;
use std::collections::HashMap;

pub fn get_local_ip() -> String {
    let local_ip_interfaces = if_addrs::get_if_addrs().unwrap();
    let local_ip = local_ip_interfaces.first().unwrap();
    local_ip.ip().to_string()
}

pub fn get_public_ip() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    let hash_values : Vec<String> = resp.values().cloned().collect();
    let public_ip = hash_values.get(0).unwrap();
    Ok(public_ip.trim().to_owned())
}