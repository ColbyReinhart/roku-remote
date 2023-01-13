use std::net::Ipv4Addr;
use serde::{Deserialize, Serialize};

// Define a device info struct
#[derive(Serialize, Deserialize, Debug)]
pub struct RokuDevice
{
    pub name: String,
    pub address: Ipv4Addr,
    pub location: String
}

impl RokuDevice
{
    pub fn new(name: String, address: Ipv4Addr, location: String) -> RokuDevice
    {
        RokuDevice { name, address, location }
    }
}