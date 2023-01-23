use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use serde::{Deserialize, Serialize};
use std::io::{Write, Error};

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

pub fn command_device(device: &SocketAddr, action: &str) -> Result<(), Error>
{	
	let mut command: TcpStream = TcpStream::connect(device)?;
	command.write("POST /keypress/".as_bytes())?;
	command.write(action.as_bytes())?;
	command.write(" HTTP/1.1\r\n\r\n".as_bytes())?;
	Ok(())
}