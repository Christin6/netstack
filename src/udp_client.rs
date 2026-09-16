use socket2::{Domain, Socket, Type};
use std::net::{SocketAddr, ToSocketAddrs};
use std::mem::MaybeUninit;

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

    let message =  "Hello, UDP server!";

    let mut buffer: [MaybeUninit<u8>; 512] = [MaybeUninit::uninit(); 512];

    match socket.connect(&address.into()) {
        Ok(_) => {
            match socket.send(message.as_bytes()) {
                Ok(_) => {
                    println!("Message sent successfully to {}", address);
                    match socket.recv(&mut buffer) {
                        Ok(n) => {
                            let bytes = unsafe {
                                std::slice::from_raw_parts(buffer.as_ptr() as *const u8, n)
                            };
                            let response = String::from_utf8_lossy(bytes);
                            println!("Received response from {}: {}", address, response);
                        }
                        Err(e) => return Err(format!("Failed to receive response: {}", e).into()),
                    }
                },
                Err(e) => return Err(format!("Failed to send message: {}", e).into()),
            }
        }
        Err(e) => return Err(format!("Failed to connect to {}: {}", address, e).into()),
    }

    Ok(())
}