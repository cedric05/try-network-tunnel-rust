use private_tunnel::parse;
use tun_tap::Iface;
mod args;
use args::Args;
use clap::Parser;
use std::io::{Read, Write};
use std::net::Ipv4Addr;
use std::str::FromStr;
fn main() {
    // Parse command line arguments
    let args = Args::parse();
    match args.mode {
        args::Mode::Client { server_ip, port } => {
            // Parse the server IP address
            let server_ip = Ipv4Addr::from_str(&server_ip).expect("Invalid server IP address");
            // Create a new TCP connection to the server
            let mut stream = std::net::TcpStream::connect((server_ip, port))
                .expect("Failed to connect to server");
            // it handles single connection. for the time being.
            println!("Connected to server at {}:{}", server_ip, port);

            // created tun interface.
            let tun: Iface = Iface::without_packet_info("tun0", tun_tap::Mode::Tun)
                .expect("Failed to create TUN interface");
            let local_ip_stream = parse::IPStream::new(tun);
            for packet in local_ip_stream {
                let packet = packet.unwrap();

                let total_length = packet.header.total_length;

                // create bytes buffer
                let mut buffer = Vec::new();
                // send length converted to u16
                buffer.extend_from_slice(&total_length.to_be_bytes());
                buffer.extend_from_slice(&packet.header_data);
                buffer.extend_from_slice(&packet.data);
                // send the packet to the server
                stream
                    .write_all(&buffer)
                    .expect("Failed to send packet to server");
                println!(
                    "packet is {:?} with lenght{:?}",
                    packet.header, packet.header.total_length
                );
            }

            // read length from first two bytes.
            // read next size. create a packet and transmit to server
        }
        args::Mode::Server { bind_ip, port } => {
            // Parse the server IP address
            let bind_ip = Ipv4Addr::from_str(&bind_ip).expect("Invalid server IP address");
            // Create a new TCP listener on the server
            let listener = std::net::TcpListener::bind((bind_ip, port))
                .expect("Failed to bind to server address");
            println!("Server listening on {}:{}", bind_ip, port);
            // Accept just single connection for now.
            let (mut stream, addr) = listener.accept().expect("Failed to accept connection");
            println!("Accepted connection from {}", addr);
            // read length from first two bytes.
            loop {
                let mut length_buffer = [0u8; 2];
                // read the first two bytes
                if stream.read_exact(&mut length_buffer).is_err() {
                    println!("Failed to read length from stream");
                    break;
                }
                // convert to u16
                let length = u16::from_be_bytes(length_buffer);
                // read the rest of the packet
                let mut packet_buffer = vec![0u8; length as usize];
                if stream.read_exact(&mut packet_buffer).is_err() {
                    println!("Failed to read packet from stream");
                    break;
                }
                println!("Received packet of length {}", length);
                println!("Packet data: {:?}", packet_buffer);
                let tun: Iface = Iface::without_packet_info("tun0", tun_tap::Mode::Tun)
                    .expect("Failed to create TUN interface");
                tun.send(&packet_buffer)
                    .expect("Failed to write packet to TUN interface");
            }
        }
    }
}
