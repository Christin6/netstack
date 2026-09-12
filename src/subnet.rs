use std::{net::{Ipv4Addr}};

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

pub fn compute_subnet_info(cidr: String) -> Result<(), Box<dyn std::error::Error>> {
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
