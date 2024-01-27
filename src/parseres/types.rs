use phf::{phf_map, Map};

pub static IP4_MIN_LEN: usize = 20;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IP4 {
    pub version: u8,
    pub ihl: u8,
    pub header_length: usize,
    pub dscp: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub dont_fragment: bool,
    pub more_fragment: bool,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub source_address: [u8;4],
    pub dist_address: [u8;4],
}

pub static IP4_TYPE: Map<&str, u8> = phf_map! {
    "TCP" => 6,
};

