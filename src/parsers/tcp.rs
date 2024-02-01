use crate::parsers::errors::TCPError;
use crate::parsers::types::{TCP, TCPFlags};

impl TCP {
    pub fn from_slice(slice: &[u8]) -> Result<TCP, TCPError>{
        let source_port = u16::from_be_bytes([slice[0], slice[1]]);
        let destination_port = u16::from_be_bytes([slice[2], slice[3]]);
        let sequence_number = u32::from_be_bytes([slice[4], slice[5], slice[6], slice[7]]);
        let ack_number = u32::from_be_bytes([slice[8], slice[9], slice[10], slice[11]]);
        let data_offset = u8::from_be_bytes([slice[12]]) & 0xf0;
        let reserved = u8::from_be_bytes([slice[12]]) & 0xf;
        let flags = Self::parse_flags(slice[13]);
        let window_size = u16::from_be_bytes([slice[14], slice[15]]);
        let checksum = u16::from_be_bytes([slice[16], slice[17]]);
        let urgent_pointer = u16::from_be_bytes([slice[18], slice[19]]);

        Ok(TCP{
            source_port,
            destination_port,
            sequence_number,
            ack_number,
            data_offset,
            reserved,
            flags,
            window_size,
            checksum,
            urgent_pointer,
        })
    }

    fn parse_flags(byte: u8) -> TCPFlags{
        TCPFlags{
            cwr: 0 != u8::from_be_bytes([byte]) & 0b1000_0000,
            ece: 0 != u8::from_be_bytes([byte]) & 0b0100_0000,
            urg: 0 != u8::from_be_bytes([byte]) & 0b0010_0000,
            ack: 0 != u8::from_be_bytes([byte]) & 0b0001_0000,
            psh: 0 != u8::from_be_bytes([byte]) & 0b0000_1000,
            rst: 0 != u8::from_be_bytes([byte]) & 0b0000_0100,
            syn: 0 != u8::from_be_bytes([byte]) & 0b0000_0010,
            fin: 0 != u8::from_be_bytes([byte]) & 0b0000_0001,
        }
    }
}