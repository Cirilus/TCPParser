use crate::parseres::errors::IP4Errors;
use crate::parseres::types::{IP4_MIN_LEN, IP4};



impl IP4 {
    pub fn from_slice(buf: &[u8]) -> Result<IP4, IP4Errors> {

        if buf.len() < IP4_MIN_LEN {
            return Err(IP4Errors::ErrMinLenPacket())
        }

        let (version, ihl) = {
            let value = u8::from_be_bytes([buf[0]]);
            (value >> 4, value & 0xf)
        };

        if version != 4 {
            return Err(IP4Errors::ErrVersion())
        }

        if ihl < 5 {
            return Err(IP4Errors::ErrorIHL())
        }

        let header_length = usize::from(ihl) * 4;
        let dscp = u8::from_be_bytes([buf[1]]) >> 2;
        let ecn = u8::from_be_bytes([buf[1]]) & 0b0000_0011;
        let total_length = u16::from_be_bytes([buf[2], buf[3]]);
        let identification = u16::from_be_bytes([buf[4], buf[4]]);
        let dont_fragment = 0 != u8::from_be_bytes([buf[6]]) & 0x40;
        let more_fragment = 0 != u8::from_be_bytes([buf[6]]) & 0x20;
        let fragment_offset = u16::from_be_bytes([buf[6] & 0x1f, buf[7]]);
        let ttl = u8::from_be_bytes([buf[8]]);
        let protocol = u8::from_be_bytes([buf[9]]);
        let header_checksum = u16::from_be_bytes([buf[10], buf[11]]);
        let source_address = [buf[12], buf[13], buf[14], buf[15]];
        let dist_address = [buf[16], buf[17], buf[18], buf[19]];

        Ok(
            IP4{
                version,
                ihl,
                header_length,
                dscp,
                ecn,
                total_length,
                identification,
                dont_fragment,
                more_fragment,
                fragment_offset,
                ttl,
                protocol,
                header_checksum,
                source_address,
                dist_address,
            }
        )
    }
}