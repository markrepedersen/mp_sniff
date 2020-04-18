pub mod arp;
pub mod blob;
pub mod dot11;
pub mod frame;
pub mod icmp;
pub mod ipv4;
pub mod ipv6;
pub mod parse;
pub mod serialize;
pub mod tcp;
pub mod udp;

use pnet::datalink;

/// Show IPv4 Packets as they come through the NIC.
pub fn show_ipv4(frame: frame::Frame) {
    if let frame::Payload::IPv4(ref ip_packet) = frame.payload {
        println!("{:#?}", ip_packet);
    }
}

/// Show IPv6 Packets as they come through the NIC.
pub fn show_ipv6(frame: frame::Frame) {
    if let frame::Payload::IPv6(ref ip_packet) = frame.payload {
        println!("{:#?}", ip_packet);
    }
}

/// Show ARP Packets as they come through the NIC.
pub fn show_arp(frame: frame::Frame) {
    if let frame::Payload::ARP(ref ip_packet) = frame.payload {
        println!("{:#?}", ip_packet);
    }
}

/// Show TCP Packets as they come through the NIC.
pub fn show_tcp(frame: frame::Frame) {
    if let frame::Payload::IPv4(ref ip_packet) = frame.payload {
        if let ipv4::Payload::TCP(ref tcp_packet) = ip_packet.payload {
            println!("{:#?}", tcp_packet);
        }
    }
}

/// Show UDP Packets as they come through the NIC.
pub fn show_udp(frame: frame::Frame) {
    if let frame::Payload::IPv4(ref ip_packet) = frame.payload {
        if let ipv4::Payload::UDP(ref udp_packet) = ip_packet.payload {
            println!("{:#?}", udp_packet);
        }
    }
}

/// Show ICMP Packets as they come through the NIC.
pub fn show_icmp(frame: frame::Frame) {
    if let frame::Payload::IPv4(ref ip_packet) = frame.payload {
        if let ipv4::Payload::ICMP(ref icmp_packet) = ip_packet.payload {
            println!("{:#?}", icmp_packet);
        }
    }
}

/// The options of which packets to display.
pub struct PacketOptions {
    pub interface: String,
    pub udp: bool,
    pub tcp: bool,
    pub icmp: bool,
    pub arp: bool,
    pub ipv4: bool,
    pub ipv6: bool,
}

/// Run an event loop, capturing the packets as described in <PacketOptions>.
pub fn run(opts: PacketOptions) {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|iface| iface.name == opts.interface)
        .next()
        .unwrap();
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Datalink channel error: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => match frame::Frame::parse(packet) {
                Ok((_remaining, frame)) => match opts {
                    PacketOptions { ipv4: true, .. } => show_ipv4(frame),
                    PacketOptions { ipv6: true, .. } => show_ipv6(frame),
                    PacketOptions { arp: true, .. } => show_arp(frame),
                    PacketOptions { tcp: true, .. } => show_tcp(frame),
                    PacketOptions { udp: true, .. } => show_udp(frame),
                    PacketOptions { icmp: true, .. } => show_icmp(frame),
                    _ => println!("{:#?}", frame),
                },
                Err(nom::Err::Error(e)) => println!("{:#?}", e),
                _ => unreachable!(),
            },
            Err(e) => {
                panic!("An error occurred while reading packet: {}", e);
            }
        }
    }
}