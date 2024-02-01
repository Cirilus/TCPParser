mod parsers;
mod types;
use tun_tap::{Iface, Mode};
use crate::parsers::types::{IP4_TYPE, Packet};
use crate::types::ETHER_TYPE;

fn main() {
    let iface = Iface::new("tcp", Mode::Tun).expect("Failed to create a TUN device");

    let mut buf = vec![0u8; 1504];

    let mut count_packet = 0;

    loop {
        let size = iface.recv(&mut buf).unwrap();
        let packet = Packet::from_slice(&buf).expect("Error parse packet");
        if packet.eth.protocol != ETHER_TYPE["IP4"] {
            // not ip4
            continue
        }

        if IP4_TYPE["TCP"] != packet.ip4.protocol {
            continue
        }
        // println!("{:#?}", packet)
    }
}

