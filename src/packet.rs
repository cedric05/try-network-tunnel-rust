use std::fmt::Display;

#[derive(Debug)]
pub struct IpHeader {
    pub version: u8,
    pub header_length: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8,
    pub fragment_offset: u16,
    pub time_to_live: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub source: [u8; 4],
    pub destination: [u8; 4],
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
pub struct IpPacket {
    pub header: IpHeader,
    pub data: Vec<u8>,
}

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
