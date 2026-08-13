use clap::{Parser, Subcommand};
use socket2::{Domain, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream};

#[derive(Parser)]
#[command(name = "netstack", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Start a TCP echo server
    EchoServer {
        #[arg(short, long, default_value_t = 7878)]
        port: u16,
    },

    // Compute subnet infor for a CIDR
    Subnet {
        cidr: String,
    },

    // Resolve a hostname by hand-crafting a DNS query over UDP
    Dig {
        hostname: String,
    },

    // Scan a host for open ports
    Scan {
        target: String, //target IP or hostname
        #[arg(long, default_value = "1-1000")]
        ports: String,
    },

    // Passively sniff traffic on a network interface
    Sniff {
        interface: String,
    },
}

fn handle_client(stream: TcpStream) {
    // enum SocketAddr
    let socket: SocketAddr = stream.peer_addr().unwrap();

    // socket.ip() returns IpAddr enum
    match socket.ip() {
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

    println!("{}", &socket.port());
}

struct UnwrappedCidr {
    ip: Ipv4Addr,
    prefix: u8,
}

fn unwrap_cidr(cidr: String) -> Result<UnwrappedCidr, Box<dyn std::error::Error>> {
    if let Some((ip_str, prefix_str)) = cidr.split_once("/") {
        let ip: Ipv4Addr = ip_str
            .parse()
            .map_err(|e| format!("Invalid IP address: {}", e))?;
        let prefix: u8 = prefix_str
            .parse()
            .map_err(|e| format!("Invalid prefix: {}", e))?;
        if prefix > 32 {
            return Err("prefix must be <= 32".into());
        }
        Ok(UnwrappedCidr { ip, prefix })
    } else {
        Err("Invalid CIDR format".into())
    }
}

fn cidr_prefix_to_mask(prefix: u8) -> Ipv4Addr {
    let mask: u32 = u32::MAX << (32 - prefix); // shift maximum binary to left by 32-prefix location
    let ipv4_mask: Ipv4Addr = Ipv4Addr::from(mask);
    return ipv4_mask;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::EchoServer { port } => {
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
        Commands::Subnet { cidr } => {
            let unwrapped = unwrap_cidr(cidr)?;
            println!(
                "IP: {} Mask: {}",
                unwrapped.ip,
                cidr_prefix_to_mask(unwrapped.prefix)
            );
            Ok(())
        }
        Commands::Dig { hostname } => {
            todo!("craft DNS query packet for '{hostname}', send over UDP, parse response")
        }
        Commands::Scan { target, ports } => {
            todo!("parse '{ports}' range, attempt connections to '{target}', report state")
        }
        Commands::Sniff { interface } => {
            todo!("open raw socket / pcap on '{interface}', print packet info as it arrives")
        }
    }
}
