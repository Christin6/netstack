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

fn to_bytes(header: &DnsHeader) -> Vec<u8> {
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

pub fn dig(hostname: String) -> Result<(), Box<dyn std::error::Error>> {
    let header = DnsHeader {
        id: 0x1234,
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

    let header_bytes = to_bytes(&header);
    println!("DNS Header Bytes: {:?}", header_bytes);

    Ok(())
}