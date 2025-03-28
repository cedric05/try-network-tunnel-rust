use std::fmt::Display;
use tun_tap::Iface;

#[derive(Debug)]
struct IpHeader {
    version: u8,
    header_length: u8,
    total_length: u16,
    identification: u16,
    flags: u8,
    fragment_offset: u16,
    time_to_live: u8,
    protocol: u8,
    header_checksum: u16,
    source: [u8; 4],
    destination: [u8; 4],
}

impl Display for IpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Version: {}", self.version)?;
        writeln!(f, "Header Length: {}", self.header_length)?;
        writeln!(f, "Total Length: {}", self.total_length)?;
        writeln!(f, "Identification: {}", self.identification)?;
        writeln!(f, "Flags: {}", self.flags)?;
        writeln!(f, "Fragment Offset: {}", self.fragment_offset)?;
        writeln!(f, "Time to Live: {}", self.time_to_live)?;
        writeln!(f, "Protocol: {}", self.protocol)?;
        writeln!(f, "Header Checksum: {}", self.header_checksum)?;
        writeln!(
            f,
            "Source: {}.{}.{}.{}",
            self.source[0], self.source[1], self.source[2], self.source[3]
        )?;
        writeln!(
            f,
            "Destination: {}.{}.{}.{}",
            self.destination[0], self.destination[1], self.destination[2], self.destination[3]
        )
    }
}

#[derive(Debug)]
struct IpPacket {
    header: IpHeader,
    data: Vec<u8>,
}

// derive display for IpPacket

impl Display for IpPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let protocol = match self.header.protocol {
            1 => "ICMP",
            2 => "IGMP",
            3 => "GGP",
            4 => "IP in IP",
            5 => "IP in IP",
            6 => "TCP",
            7 => "CBT",
            8 => "EGP",
            9 => "IGP",
            10 => "BBN-RCC",
            11 => "NVP-II",
            12 => "PUP",
            13 => "ARGUS",
            14 => "EMCON",
            15 => "XNET",
            16 => "CHAOS",
            17 => "UDP",
            18 => "MUX",
            19 => "DCN",
            20 => "HMP",
            21 => "PRM",
            22 => "XNS-IDP",
            23 => "TRUNK-1",
            24 => "TRUNK-2",
            25 => "LEAF-1",
            26 => "LEAF-2",
            27 => "RDP",
            28 => "IRTP",
            29 => "ISO-TP4",
            30 => "NETBLT",
            31 => "MFE-NSP",
            32 => "MERIT-INP",
            33 => "DCCP",
            34 => "3PC",
            35 => "IDPR",
            36 => "XTP",
            89 => "OSPF",
            132 => "SCTP",
            _ => "Unknown Protocol",
        };
        writeln!(f, "Protocol {}, data: {:?}", protocol, self.data)
    }
}

enum CurrentState {
    Header,
    Data(u16),
}

impl CurrentState {
    fn needed_size(&self) -> usize {
        match self {
            CurrentState::Header => 20,
            CurrentState::Data(size) => *size as usize,
        }
    }
}

fn main() -> std::io::Result<()> {
    let tun: Iface = Iface::without_packet_info("tun0", tun_tap::Mode::Tun)
        .expect("Failed to create TUN interface");

    let mut buffer = [0u8; 1504];
    let mut pending = Vec::new();
    let mut current_state = CurrentState::Header;
    let mut current_header = None;

    loop {
        let bytes_read = tun.recv(&mut buffer)?;
        let data = &buffer[..bytes_read];

        if pending.is_empty() {
            pending = data.to_vec();
        } else {
            pending.extend_from_slice(data);
        }

        match current_state {
            CurrentState::Header => {
                let currect_header_size = ((pending[0] & 0x0f) * 4) as usize;
                if pending.len() < currect_header_size.into() {
                    println!("Packet too short to be an IP packet");
                    continue;
                }
                let header = &pending[..currect_header_size];
                let ip_header = parse_ip_header(header);
                println!("{}", ip_header);

                let total_length = ip_header.total_length;
                if total_length > currect_header_size as u16 {
                    current_state = CurrentState::Data(total_length - currect_header_size as u16);
                }
                pending.drain(..currect_header_size);
                current_header = Some(ip_header);
            }
            CurrentState::Data(_) => {
                let needed_size = current_state.needed_size();
                if pending.len() < needed_size {
                    continue;
                }

                let packet = IpPacket {
                    header: current_header.take().unwrap(),
                    data: pending[..needed_size].to_vec(),
                };

                println!("Packet: {}", packet);
                current_state = CurrentState::Header;
                pending.drain(..needed_size);
            }
        }
    }
}

fn parse_ip_header(header: &[u8]) -> IpHeader {
    IpHeader {
        version: (header[0] >> 4) & 0x0F,
        header_length: (header[0] & 0x0F) * 4,
        total_length: u16::from_be_bytes([header[2], header[3]]),
        identification: u16::from_be_bytes([header[4], header[5]]),
        flags: (header[6] >> 5) & 0x07,
        fragment_offset: u16::from_be_bytes([header[6] & 0x1F, header[7]]),
        time_to_live: header[8],
        protocol: header[9],
        header_checksum: u16::from_be_bytes([header[10], header[11]]),
        source: [header[12], header[13], header[14], header[15]],
        destination: [header[16], header[17], header[18], header[19]],
    }
}
