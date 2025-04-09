use std::{error::Error, fmt::Display};

use crate::packet::{IpHeader, IpPacket};
use tun_tap::Iface;

enum CurrentState {
    Header(Option<u8>),
    Data(u16),
}
impl CurrentState {
    fn header(size: u8) -> Self {
        Self::Header(Some(size))
    }
}

#[derive(Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "parse Error")
    }
}
impl Error for ParseError {}

pub struct IPStream {
    tun: Iface,
    buffer: Vec<u8>,
    current_state: CurrentState,
}

impl Iterator for IPStream {
    type Item = Result<IpPacket, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut single_buf = [0u8; 1504];
        let mut current_header = None;

        loop {
            let bytes_read = match self.tun.recv(&mut single_buf) {
                Ok(bytes_read) => bytes_read,
                Err(_) => return Some(Err(ParseError)),
            };
            let data = &single_buf[..bytes_read];

            if self.buffer.is_empty() {
                self.buffer = data.to_vec();
            } else {
                self.buffer.extend_from_slice(data);
            }

            if let CurrentState::Header(header_size) = self.current_state {
                let currect_header_size = match header_size {
                    Some(size) => size,
                    None => {
                        if self.buffer.is_empty() {
                            20
                        } else {
                            let current_header_size = (self.buffer[0] & 0x0f) * 4;
                            self.current_state = CurrentState::header(current_header_size);
                            current_header_size
                        }
                    }
                };
                if self.buffer.len() < currect_header_size.into() {
                    continue;
                }
                let header_buffer: Vec<u8> = self
                    .buffer
                    .drain(..(currect_header_size as usize))
                    .collect();
                let ip_header = parse_ip_header(&header_buffer);
                let total_packet_length = ip_header.total_length;
                if total_packet_length >= currect_header_size as u16 {
                    self.current_state =
                        CurrentState::Data(total_packet_length - currect_header_size as u16);
                };
                current_header = Some((ip_header, header_buffer));
            }
            if let CurrentState::Data(size) = self.current_state {
                let needed_size = size as usize;
                if self.buffer.len() < needed_size {
                    continue;
                }
                let data: Vec<u8> = self.buffer.drain(..needed_size).collect();
                let (header, header_data) = current_header.take().unwrap();
                let packet = IpPacket {
                    header,
                    header_data,
                    data,
                };
                self.current_state = CurrentState::Header(None);
                return Some(Ok(packet));
            }
        }
    }
}

impl IPStream {
    pub fn new(tun: Iface) -> Self {
        Self {
            tun,
            buffer: Vec::new(),
            current_state: CurrentState::Header(None),
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
