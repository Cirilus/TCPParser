mod parseres;
mod types;

use tun_tap::{Iface, Mode};
use crate::types::{ETHER_TYPE};
use parseres::types::{IP4, IP4_TYPE};

fn main() {
    let iface = Iface::new("tcp", Mode::Tun).expect("Failed to create a TUN device");

    let mut buf = vec![0u8; 1504];

    let mut count_packet = 0;

    loop {
        let size = iface.recv(&mut buf).unwrap();
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != ETHER_TYPE["IP4"] {
            // not ip4
            continue
        }

        let ip4 = IP4::from_slice(&buf[4..]).expect("Error IP4 parsing");

        if IP4_TYPE["TCP"] != ip4.protocol {
            continue
        }

        count_packet += 1;

        println!("the packet {:?} size of packet {:?}, ip4 {:?}\n",
                 count_packet, size, ip4);
    }
}

