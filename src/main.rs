// Roku Remote Webserver
// By Colby Reinhart
// 1-10-2023

use std::
{
	net::{IpAddr, Ipv4Addr, TcpStream, TcpListener, SocketAddr},
	io::{Write, Read},
	time::Duration,
	str::Split, fs::read_to_string,
};

use local_ip_address::local_ip;
use roku_remote::RokuDevice;
use roxmltree::Document;
use http_request_parser::{Request, Response};

static SUBNET_SEARCH_LIMIT: u8 = 15;	// Last number of subnet to check (exclusive)

fn main()
{
	// First, search the current subnet for Roku devices
	let devices: Vec<RokuDevice> = find_devices();
	///let devices: Vec<RokuDevice> = Vec::new();///
	for device in &devices
	{
		println!("{:?}", device);
	}

	// Create a socket to listen for requests on LAN
	let listener: TcpListener = TcpListener::bind("0.0.0.0:5000").unwrap();

    for stream in listener.incoming()
	{
        let mut stream: TcpStream = stream.unwrap();
        handle_request(&mut stream, &devices);
    }
}

// Handle a request
fn handle_request(stream: &mut TcpStream, devices: & Vec<RokuDevice>)
{
	// Read in and parse the request
	let req: Request = Request::from(&stream);
	let mut res: Response = Response::new();

	println!("{:?}", req);

	// If they sent a keypress request, find out what key and which device
	// to send it to
	if req.method == "PUT" && req.path == "/keypress"
	{	
		// What's the device and what key should be pressed?
		let mut device: &str = "NULL";
		let mut action: &str = "NULL";

		// We'll assume we're getting a standard form body
		for form_input in req.body.split("&").into_iter()
		{
			// Get the name and value of each input
			let pair: Vec<&str> = form_input.split("=").collect();

			// Assert valid syntax
			if pair.len() != 2
			{
				res.status = 400;
				res.status_message = format!("Invalid PUT parameter: {}", pair.join("="));
				res.send(stream);
				return;
			}

			match pair.get(0).expect("Could not inspect PUT body parameter")
			{
				&"device" => device = pair.get(1).unwrap(),
				&"action" => action = pair.get(1).unwrap(),
				_ => () // Inclusion of additional form data is unnecessary but
						// shouldn't break anything
			}
		}

		// Verify that we have the data we need
		if device == "NULL" || action == "NULL"
		{
			res.status = 400;
			res.status_message = "Incorrect parameters for keypress call!".to_owned();
			res.send(stream);
			return;
		}

		// Now that we have the device and action, lets get the info we need
		// and send out the command.
		let device_info: Vec<&RokuDevice> = devices.
			into_iter()
			.filter(|roku| roku.name == device)
			.collect();

		// Handle the case where the device doesn't exist
		if device_info.is_empty()
		{
			res.status = 400;
			res.status_message = format!("No device found with name: {}", device);
			res.send(stream);
			return;
		}

		// We'll assume that Rokus can't have the same device name. If this assumption is wrong,
		// then whichever Roku has the lower IP address (was inserted into the vector first)
		// will be issued the command
		let device_to_command: &&RokuDevice = device_info.get(0).unwrap();
		let device_socket: SocketAddr =
			SocketAddr::new(IpAddr::V4(device_to_command.address), 8060);
		
		// Write the request
		let mut command: TcpStream = TcpStream::connect(device_socket).unwrap();
		command.write("POST /keypress/".as_bytes()).unwrap();
		command.write(action.as_bytes()).unwrap();
		command.write(" HTTP/1.1\r\n\r\n".as_bytes()).unwrap();

		// Listen for the response and send it along to the client
		let mut command_res: String = String::new();
		command.read_to_string(&mut command_res).unwrap();
		
		println!("{}", command_res);

		stream.write(command_res.as_bytes()).unwrap();
	}
	// A GET to root will just serve the HTML
	else if req.method == "GET" && req.path == "/"
	{
		res.status = 200;

		// Read in index, set as the response body, and send it along
		let file: String = read_to_string("static/index.html")
			.expect("Could not open index.html");
		res.headers.push("Content-Type: text/html; charset=utf-8".to_owned());
		res.body = file;
		res.send(stream);
	}
	// A GET to /devices will return a list of device names
	else if req.method == "GET" && req.path == "/devices"
	{
		res.status = 200;
		let device_names: Vec<String> = devices.into_iter()
			.map(|f| f.name.to_owned())
			.collect();
		res.body = device_names.join(",");
		res.send(stream);
	}
	// Everything else is a 404
	else
	{
		res.status = 404;
		res.status_message = "Not Found".to_owned();
		res.send(stream);
	}
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