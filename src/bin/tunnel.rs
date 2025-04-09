use private_tunnel::parse;
use tun_tap::Iface;

fn main() {
    let tun: Iface = Iface::without_packet_info("tun0", tun_tap::Mode::Tun)
        .expect("Failed to create TUN interface");
    let stream = parse::IPStream::new(tun);
    for packet in stream {
        let packet = packet.unwrap();
        println!(
            "packet is {:?} with lenght{:?}",
            packet.header, packet.header.total_length
        );
    }
}
