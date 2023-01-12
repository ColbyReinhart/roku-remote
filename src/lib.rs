use std::net::Ipv4Addr;

// Define a device info struct
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