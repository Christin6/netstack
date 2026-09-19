use clap::{Parser, Subcommand};

mod echo_server;
mod subnet;
mod tcp_client;
mod udp_client;
mod echo_udp_server;
mod dns;

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

    // Start a UDP echo server
    EchoUdpServer {
        #[arg(short, long, default_value_t = 7878)]
        port: u16,
    },

    // Create a TCP client that connects to a server and sends a message
    CreateTcpClient {
        #[arg(long, default_value = "localhost")]
        host: String,
        #[arg(short, long, default_value_t = 7878)]
        port: u16,
    },

    // Create a UDP client that sends a message to a server
    CreateUdpClient {
        #[arg(long, default_value = "localhost")]
        host: String,
        #[arg(short, long, default_value_t = 7878)]
        port: u16,
    },

    // Compute subnet information for a CIDR
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
        #[arg(long, default_value = "1-1000")] // no short because -p is already used for singular port
        ports: String,
    },

    // Passively sniff traffic on a network interface
    Sniff {
        interface: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::EchoServer { port } => {
            echo_server::echo_server(port)?;
            Ok(())
        }
        Commands::EchoUdpServer { port } => {
            echo_udp_server::echo_udp_server(port)?;
            Ok(())
        }
        Commands::CreateTcpClient { host, port } => {
            tcp_client::create_tcp_client(host, port)?;
            Ok(())
        }
        Commands::CreateUdpClient { host, port } => {
            udp_client::create_udp_client(host, port)?;
            Ok(())
        }
        Commands::Subnet { cidr } => {
            subnet::compute_subnet_info(cidr)?;
            Ok(())
        }
        Commands::Dig { hostname } => {
            dns::dig(hostname)?;
            Ok(())
        }
        Commands::Scan { target, ports } => {
            todo!("parse '{ports}' range, attempt connections to '{target}', report state")
        }
        Commands::Sniff { interface } => {
            todo!("open raw socket / pcap on '{interface}', print packet info as it arrives")
        }
    }
}
