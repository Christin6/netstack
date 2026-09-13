use socket2::{Domain, Socket, Type};
use std::io::Read;
use std::{net::{IpAddr, SocketAddr, TcpListener, TcpStream}};

fn print_incoming_device_info(ip_addr: IpAddr, port: u16) {
    match ip_addr {
        // pull out the value of IpAddr enum
        IpAddr::V4(original_ip) => {
            // IpAddr::V4 wraps an Ipv4Addr directly
            // original_ip is already IPv4Addr
            println!("v4: {}", &original_ip);
        }
        IpAddr::V6(original_ip) => {
            // IpAddr::V6 wraps an Ipv6Addr directly
            println!("v6: {}", &original_ip);
            if let Some(ipv4) = original_ip.to_ipv4_mapped() {
                // pull out the value of Option<Ipv4Addr>
                println!("v4 mapped: {}", &ipv4);
            }
        }
    }
    println!("{}", port);
}

fn handle_client(mut stream: TcpStream) {
    // handle a single client connection
    // enum SocketAddr
    let socket: SocketAddr = stream.peer_addr().unwrap();

    // socket.ip() returns IpAddr enum, socket.port() returns u16
    print_incoming_device_info(socket.ip(), socket.port());

    // read from the stream and print the message
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // connection closed
                println!("\n---\n");
                break;
            }
            Ok(n) => {
                // print the message received from the client
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("{}", message);
            }
            Err(e) => {
                println!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
}

pub fn echo_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let socket = Socket::new(Domain::IPV6, Type::STREAM, None)?;

    socket.set_only_v6(false)?;
    let address: SocketAddr = format!("[::]:{port}").parse()?;
    socket.bind(&address.into())?;
    socket.listen(128)?;

    let listener: TcpListener = socket.into();
    println!("Server listening on port {port}");

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
