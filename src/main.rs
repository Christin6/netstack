use clap::{Parser, Subcommand};
use socket2::{Domain, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

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

fn ip_to_u32(ip: Ipv4Addr) -> u32 {
    u32::from(ip)
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
    if prefix == 0 {
        return Ipv4Addr::new(0, 0, 0, 0);
    }
    let mask: u32 = u32::MAX << (32 - prefix); // shift maximum binary to left by 32-prefix location
    let ipv4_mask: Ipv4Addr = Ipv4Addr::from(mask);
    return ipv4_mask;
}

fn compute_network_address(ip: Ipv4Addr, mask: Ipv4Addr) -> Ipv4Addr {
    let ip_u32: u32 = ip_to_u32(ip);
    let mask_u32: u32 = ip_to_u32(mask);
    let network_u32: u32 = ip_u32 & mask_u32;
    Ipv4Addr::from(network_u32)
}

fn compute_broadcast_address(network_address: Ipv4Addr, mask: Ipv4Addr) -> Ipv4Addr {
    let network_u32: u32 = ip_to_u32(network_address);
    let mask_u32: u32 = ip_to_u32(mask);
    let broadcast_u32: u32 = network_u32 | !mask_u32;
    Ipv4Addr::from(broadcast_u32)
}

struct HostRange {
    first_usable: Ipv4Addr,
    last_usable: Ipv4Addr,
    host_count: u32,
}

fn compute_host_range(network_address: Ipv4Addr, broadcast_address: Ipv4Addr) -> HostRange {
    let network_u32: u32 = ip_to_u32(network_address);
    let broadcast_u32: u32 = ip_to_u32(broadcast_address);
    let first_usable_u32: u32 = network_u32 + 1;
    let last_usable_u32: u32 = broadcast_u32 - 1;
    let host_count: u32 = (last_usable_u32 - first_usable_u32 + 1) as u32;
    HostRange {
        first_usable: Ipv4Addr::from(first_usable_u32),
        last_usable: Ipv4Addr::from(last_usable_u32),
        host_count,
    }
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
            let unwrapped: UnwrappedCidr = unwrap_cidr(cidr)?;
            let cidr_mask: Ipv4Addr = cidr_prefix_to_mask(unwrapped.prefix);
            let network_address: Ipv4Addr = compute_network_address(unwrapped.ip, cidr_mask);
            let broadcast_address: Ipv4Addr = compute_broadcast_address(network_address, cidr_mask);
            let host_range: HostRange = if unwrapped.prefix == 32 {
                HostRange {
                    first_usable: unwrapped.ip,
                    last_usable: unwrapped.ip,
                    host_count: 1,
                }
            } else if unwrapped.prefix == 31 {
                HostRange {
                    first_usable: network_address,
                    last_usable: broadcast_address,
                    host_count: 2,
                }
            } else {
                compute_host_range(network_address, broadcast_address)
            };

            println!("IP: {} Mask: {}", unwrapped.ip, cidr_mask);

            println!("Network address: {}", network_address);

            println!("Broadcast address: {}", broadcast_address);

            println!(
                "Host range: {} - {} ({} hosts)",
                host_range.first_usable, host_range.last_usable, host_range.host_count
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
