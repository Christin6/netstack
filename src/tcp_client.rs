use socket2::{Domain, Socket, Type};
use std::{net::{SocketAddr, ToSocketAddrs}};
use std::io::{ErrorKind};

pub fn create_tcp_client(host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let socket: Socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    let address: SocketAddr = match format!("{host}:{port}").to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(addr) => addr,
            None => return Err(format!("No valid address found for {host}:{port}").into()),
        },
        Err(error) => return Err(format!("Failed to parse address {host} on port {port}: {error}").into()),
    };

    match socket.connect(&address.into()) {
        Ok(_) => {
            println!("Successfully connected to {host}:{port}");
            Ok(())
        },
        Err(error) => {
            if error.kind() == ErrorKind::ConnectionRefused {
                Err(format!("Connection refused: Is the server running on {host}:{port}?").into())
            } else if error.kind() == ErrorKind::TimedOut {
                Err(format!("Connection timed out: Could not connect to {host}:{port}").into())
            } else {
                Err(format!("Failed to connect to {host}:{port}: {error}").into())
            }
        }
    }
}