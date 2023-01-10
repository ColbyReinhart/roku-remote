// Roku Remote Webserver
// By Colby Reinhart
// 1-10-2023

use std::
{
	net::{UdpSocket, Ipv4Addr},
	io::BufReader,
	fs::File, time::Duration
};

fn main()
{
	find_devices();
}

fn find_devices()
{
	// Set up a UDP socket
	let socket: UdpSocket = UdpSocket::bind("127.0.0.1:5000")
		.expect("Server couldn't bind to address");
	println!("Set up socket");

	// Configure socket
	socket.join_multicast_v4(&Ipv4Addr::new(239, 255, 255, 250), &Ipv4Addr::UNSPECIFIED).unwrap();
	socket.set_read_timeout(Some(Duration::new(10, 0))).unwrap(); // Only wait 10 seconds

	// Buffer in the request body of the multicast message
	let multicast_body: File = File::open("static/roku-ecp-req.txt").unwrap();
	let buffer: BufReader<File> = BufReader::new(multicast_body);
	println!("Read in multicast request body");

	// Send the multicast and get the result
	socket.send_to(buffer.buffer(), "239.255.255.250:1900").unwrap();
	println!("Sent multicast");

	let mut buf = [0u8; 256];
	match socket.recv_from(&mut buf)
	{
		Ok((len, remote_addr)) =>
		{
			let data = &buf[..len];
			let response = String::from_utf8_lossy(data);
			println!("Got response from {}", remote_addr);
			println!("{}", response);
		}
		Err(err) =>
		{
			println!("{}", err);
		}
	}
}