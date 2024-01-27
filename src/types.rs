use phf::{Map, phf_map};

pub static ETHER_TYPE: Map<&str, u16> = phf_map! {
    "IP4" => 0x0800,
};
