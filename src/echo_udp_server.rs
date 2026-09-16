use socket2::{Domain, Socket, Type};
use std::{mem::MaybeUninit, net::SocketAddr};

pub fn echo_udp_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let socket = Socket::new(Domain::IPV6, Type::DGRAM, None)?;

    socket.set_only_v6(false)?;
    let address: SocketAddr = format!("[::]:{port}").parse()?;
    socket.bind(&address.into())?;

    let mut buffer: [MaybeUninit<u8>; 512] = [MaybeUninit::uninit(); 512];

    let echo_message = "Hello, UDP client!";

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((n, addr)) => {
                let bytes = unsafe {
                    std::slice::from_raw_parts(buffer.as_ptr() as *const u8, n)
                };
                let message = String::from_utf8_lossy(bytes);
                println!("Received from {}: {}", &addr.as_socket().unwrap(), message);

                let addr_as_string = addr.as_socket().unwrap().to_string();

                // Echo the message back to the client
                match socket.send_to(echo_message.as_bytes(), &addr.into()) {
                    Ok(_) => println!("Message echoed back to {}", &addr_as_string),
                    Err(e) => println!("Failed to echo message: {}", e),
                }
            }
            Err(e) => {
                println!("Failed to receive data: {}", e);
                break;
            }
        }
    }

    Ok(())
}