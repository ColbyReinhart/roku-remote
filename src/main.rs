// Roku Remote Webserver
// By Colby Reinhart
// 1-10-2023

use std::{net::{IpAddr, Ipv4Addr, TcpStream, SocketAddr}, io::{Write, Read}};

use local_ip_address::local_ip;

fn main()
{
	find_devices();
}

// Discover devices on the LAN
fn find_devices()
{
	// Roku API documentation says to use SSDP, but I can't get it to work
	// correctly. Instead, we'll send a /query/device-info API request to
	// each reachable endpoint on the network and consider any valid responses
	// as a roku device.
	// TVs seem to not respond to network communication if they've been
	// powered off for a while. Idk how the remote app gets around all
	// these issues, but I wish it was documented.

	// To scan every device on the current subnet, we'll find the host
	// machine's current IP address. I know this is bad, but we'll just
	// assume that the subnet mask is 24, since that applies to 99% of
	// home networks.

	let host_ip: IpAddr = local_ip().expect("Couldn't determine host ip");
	let octets: [u8; 4] = match host_ip
	{
		IpAddr::V4(ip4) => ip4.octets(),
		IpAddr::V6(_) => panic!() // This shouldn't ever happen
	};
	
	// For all possible device addresses
	for i in 1u8..254u8
	{
		// Construct the address
		let addr: SocketAddr = SocketAddr::new
		(
			IpAddr::V4(Ipv4Addr::new(octets[0], octets[1], octets[2], i)),
			8060
		);

		println!("Testing {:?}", addr);
		
		// Connect to the address
		if let Ok(mut stream) = TcpStream::connect(addr)
		{
			// If we connected successfully, try to query device info
			stream.write(&"GET /query/device-info HTTP/1.1\r\n\r\n".as_bytes())
				.unwrap();
			let mut buf: String = String::new();
			stream.read_to_string(&mut buf).unwrap();

			println!("{}", buf);
		}
	}
}