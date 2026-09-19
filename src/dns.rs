use std::time::SystemTime;
use crate::udp_client;

// https://www.rfc-editor.org/info/rfc1035/#section-4.1.1
struct DnsHeader {
    id: u16,
    qr: bool,
    opcode: u8,
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    z: u8,
    rcode: u8,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

fn header_to_bytes(header: &DnsHeader) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(12);
    bytes.extend_from_slice(&header.id.to_be_bytes());
    let flags = ((header.qr as u16) << 15)
        | ((header.opcode as u16) << 11)
        | ((header.aa as u16) << 10)
        | ((header.tc as u16) << 9)
        | ((header.rd as u16) << 8)
        | ((header.ra as u16) << 7)
        | ((header.z as u16) << 4)
        | (header.rcode as u16);
    bytes.extend_from_slice(&flags.to_be_bytes());
    bytes.extend_from_slice(&header.qdcount.to_be_bytes());
    bytes.extend_from_slice(&header.ancount.to_be_bytes());
    bytes.extend_from_slice(&header.nscount.to_be_bytes());
    bytes.extend_from_slice(&header.arcount.to_be_bytes());
    bytes
}

// https://www.rfc-editor.org/info/rfc1035/#section-4.1.2
fn hostname_to_bytes(string: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let labels: Vec<&str> = string.split('.').collect();
    let mut bytes = Vec::new();
    for label in labels {
        if label.len() > 63 {
            return Err("Label is too long".into());
        }
        bytes.push(label.len() as u8);
        bytes.extend_from_slice(label.as_bytes());
    }
    bytes.push(0); // Null terminator for the last label
    Ok(bytes)
}

fn question_to_bytes(hostname: &str, qtype: u16, qclass: u16) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut bytes = hostname_to_bytes(hostname)?;
    bytes.extend_from_slice(&qtype.to_be_bytes());
    bytes.extend_from_slice(&qclass.to_be_bytes());
    Ok(bytes)
}

const QTYPE_A: u16 = 1;
const QCLASS_IN: u16 = 1;

fn build_dns_query(hostname: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // use the current time as a simple transaction ID (vulnerable against DNS cache poisoning)
    let id_from_time: u16 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u16;

    let header = DnsHeader {
        id: id_from_time,
        qr: false,
        opcode: 0,
        aa: false,
        tc: false,
        rd: true,
        ra: false,
        z: 0,
        rcode: 0,
        qdcount: 1,
        ancount: 0,
        nscount: 0,
        arcount: 0,
    };

    let mut query = header_to_bytes(&header);
    let question = question_to_bytes(hostname, QTYPE_A, QCLASS_IN)?;
    query.extend_from_slice(&question);
    Ok(query)
}

pub fn dig(hostname: String) -> Result<(), Box<dyn std::error::Error>> {
    let query = build_dns_query(&hostname)?;
    println!("DNS Query Bytes: {:?}", query);

    let socket: Result<udp_client::UdpSocket, Box<dyn std::error::Error>> = udp_client::create_udp("8.8.8.8".to_owned(), 53);
    let udp_client::UdpSocket {socket, address} = socket.unwrap();
    let response = udp_client::send_and_receive(&socket, &address, &query);
    println!("{:?}", response?);

    Ok(())
}