use clap::{Parser, Subcommand};
use socket2::{Domain, Socket, Type};
use std::net::{SocketAddr, TcpListener};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::EchoServer { port } => {
            let socket = Socket::new(Domain::IPV6, Type::STREAM, None)?;

            socket.set_only_v6(false)?;
            let address: SocketAddr = format!("[::1]:{port}").parse()?;
            socket.bind(&address.into())?;
            socket.listen(128)?;

            let listener: TcpListener = socket.into();
            println!("Server listening on port {port}");
            println!("{listener:?}");
            Ok(())
        }
        Commands::Subnet { cidr } => {
            todo!("parse '{cidr}', compute network/broadcast/host range/count")
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
