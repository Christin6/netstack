use socket2::{Domain, Socket, Type};
use std::net::{SocketAddr, ToSocketAddrs};

pub fn create_udp_client(host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let socket: Socket = Socket::new(Domain::IPV4, Type::DGRAM, None)?;
    let address: SocketAddr = match format!("{host}:{port}").to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(addr) => addr,
            None => return Err(format!("No valid address found for {host}:{port}").into()),
        },
        Err(error) => {
            return Err(format!("Failed to parse address {host} on port {port}: {error}").into());
        }
    };
    Ok(())
}