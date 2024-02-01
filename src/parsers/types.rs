use phf::{phf_map, Map};

pub static IP4_MIN_LEN: usize = 20;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Packet {
    pub eth: Ethernet,
    pub ip4: IP4,
    pub tcp: TCP
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ethernet {
    pub flags: u16,
    pub protocol: u16,
}

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TCP {
    pub source_port: u16,
    pub destination_port: u16,
    pub sequence_number: u32,
    pub ack_number: u32,
    pub data_offset: u8,
    pub reserved: u8,
    pub flags: TCPFlags,
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TCPFlags {
    pub cwr: bool,
    pub ece: bool,
    pub urg: bool,
    pub ack: bool,
    pub psh: bool,
    pub rst: bool,
    pub syn: bool,
    pub fin: bool,
}

pub static IP4_TYPE: Map<&str, u8> = phf_map! {
    "TCP" => 6,
};

