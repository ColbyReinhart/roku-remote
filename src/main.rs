// Roku Remote Webserver
// By Colby Reinhart
// 1-10-2023

use std::
{
	net::{IpAddr, Ipv4Addr, TcpStream, TcpListener, SocketAddr},
	io::{Write, Read, BufReader},
	time::Duration,
	str::Split,
};

use local_ip_address::local_ip;
use roku_remote::RokuDevice;
use roxmltree::Document;
use httparse::{Header, Request};

static SUBNET_SEARCH_LIMIT: u8 = 15;	// Last number of subnet to check (exclusive)

fn main()
{
	// First, search the current subnet for Roku devices
	let devices: Vec<RokuDevice> = find_devices();
	for device in &devices
	{
		println!("{:?}", device);
	}

	// Create a socket to listen for requests on LAN
	let listener: TcpListener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming()
	{
        let stream: TcpStream = stream.unwrap();

        handle_request(stream, &devices);
    }
}

// Handle a request
fn handle_request(mut stream: TcpStream, devices: & Vec<RokuDevice>)
{
	// Read out the request to a string
	let mut buf_reader = BufReader::new(&mut stream);
	let mut buf: String = String::new();
	buf_reader.read_to_string(&mut buf).unwrap();

	// Parse out the request
	let mut headers : [Header; 64] = [httparse::EMPTY_HEADER; 64];
	let mut req: Request = httparse::Request::new(&mut headers);
	
	// TODO: Should I use a match here?

}


// Discover devices on the LAN
fn find_devices() -> Vec<RokuDevice>
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
	
	let mut found_devices: Vec<RokuDevice> = Vec::new();

	// For all possible device addresses
	for i in 1u8..SUBNET_SEARCH_LIMIT
	{
		// Construct the address
		let ip: Ipv4Addr = Ipv4Addr::new(octets[0], octets[1], octets[2], i);
		let addr: SocketAddr = SocketAddr::new
		(
			IpAddr::V4(ip),
			8060
		);

		println!("Testing {:?}", addr);
		
		// Connect to the address
		if let Ok(mut stream) =
			TcpStream::connect_timeout(&addr, Duration::new(3, 0))
		{
			// If we connected successfully, try to query device info
			stream.write(&"GET /query/device-info HTTP/1.1\r\n\r\n".as_bytes())
				.unwrap();
			let mut buf: String = String::new();
			stream.read_to_string(&mut buf).unwrap();

			// Parse the response. Start by separating the head and body. We're assuming
			// the response complies with HTTP, because I'm lazy
			// TODO: Use httparse here!
			let mut head_and_body: Split<&str> = buf.split("\r\n\r\n");
			let head: &str = head_and_body.next().expect("Response does not have valid header");
			let body: &str = head_and_body.next().expect("Response does not have valid body");

			// If we got a 200 response, we'll assume the body is xml
			let status: &str = head.lines().into_iter().next().unwrap();
			if status.contains("200")
			{
				println!("Found a device!");
				
				let xml: Document = Document::parse(body).unwrap();
				let name: &str = xml.descendants()
					.find(|n| n.tag_name().name() == "friendly-device-name")
					.unwrap()
					.text()
					.unwrap();
				let location: &str = xml.descendants()
					.find(|n| n.tag_name().name() == "user-device-location")
					.unwrap()
					.text()
					.unwrap();
				
				found_devices.push(RokuDevice::new
				(
					name.to_string(), 
					ip,
					location.to_string()
				));
			}
		}
	}

	found_devices
}