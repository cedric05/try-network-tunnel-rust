use network_stuf::parse;
use tun_tap::Iface;

fn main() {
    let tun: Iface = Iface::without_packet_info("tun0", tun_tap::Mode::Tun)
        .expect("Failed to create TUN interface");
    let stream = parse::IPStream::new(tun);
    for packet in stream {
        println!("packet is {packet:?}");
    }
}
