use phf::{phf_map, Map};

pub static ETHER_TYPE: Map<&str, u16> = phf_map! {
    "IP4" => 0x0800,
};

