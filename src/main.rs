use clap::{Parser, Subcommand};

mod echo_server;
mod subnet;

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
            echo_server::echo_server(port)?;
            Ok(())
        }
        Commands::Subnet { cidr } => {
            subnet::compute_subnet_info(cidr)?;
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
