use pnet::datalink::NetworkInterface;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InterfaceDto {
    pub name: String,
    pub description: String,
    pub mac: String,
    pub ips: Vec<String>,
    pub up: bool,
    pub loopback: bool,
    pub running: bool
}

impl InterfaceDto {
    pub fn from_network_interface(intf: &NetworkInterface) -> InterfaceDto {
        return InterfaceDto {
            name: intf.name.clone(),
            description: intf.description.clone(),
            mac: intf.mac
                .map(|m| m.to_string())
                .unwrap_or(String::from("unknown")),
            ips: intf.ips.iter()
                .map(|ip| ip.to_string())
                .collect(),
            up: intf.is_up(),
            loopback: intf.is_loopback(),
            running: intf.is_running()
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TablesResponse {
    pub tables: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ChainsResponse {
    pub chains: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RulesResponse {
    pub rules: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DataMap<T> {
    pub data: T
}