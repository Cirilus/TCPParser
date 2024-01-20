mod types;

use std::u8;
use tun_tap::{Iface, Mode};
use crate::types::ETHER_TYPE;

fn main() {
    let iface = Iface::new("tcp", Mode::Tun).expect("Failed to create a TUN device");

    let mut buf = vec![0u8; 1504];


    loop {
        let size = iface.recv(&mut buf).unwrap();
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != ETHER_TYPE["IP4"] {
            // not ip4
            continue
        }

        let (version, ihl) = {
            let value = u8::from_be_bytes([buf[4]]);
            (value >> 4, value & 0xf)
        };

        if version != 4 {
            panic!("Error version");
        }

        if ihl < 5 {
            panic!("Error Internet header length");
        }

        let header_length = usize::from(ihl) * 4;
        let dscp = u8::from_be_bytes([buf[5]]) >> 2;
        let ecn = u8::from_be_bytes([buf[5]]) & 0b0000_0011;
        let total_length = u16::from_be_bytes([buf[5], buf[6]]);

        println!("size of packet {:?}, version: {:?}, ihl: {:?}, \
        header_length: {:?}, dcp: {:?}, \
        enc: {:?}, total_length: {:?}",
                 size, version, ihl,
                 header_length, dscp,
                 ecn, total_length);
    }
}

