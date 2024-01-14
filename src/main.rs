use tun_tap::{Iface, Mode};

fn main() {
    let iface = Iface::new("tcp", Mode::Tun).expect("Failed to create a TUN device");

    let mut buf = vec![0u8; 1504];


    loop {
        let size = iface.recv(&mut buf).unwrap();
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            // not ip4
            continue
        }

        println!("{:?}", &buf[4..size]);

    }






}
