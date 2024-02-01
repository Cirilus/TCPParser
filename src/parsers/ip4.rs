use crate::parsers::errors::IP4Error;
use crate::parsers::types::{IP4_MIN_LEN, IP4};
impl IP4 {
    pub fn from_slice(slice: &[u8]) -> Result<IP4, IP4Error> {

        if slice.len() < IP4_MIN_LEN {
            return Err(IP4Error::ErrMinLenPacket)
        }

        let (version, ihl) = {
            let value = u8::from_be_bytes([slice[0]]);
            (value >> 4, value & 0xf)
        };

        // if version != 4 {
        //     return Err(IP4Error::ErrVersion)
        // }
        //
        // if ihl < 5 {
        //     return Err(IP4Error::ErrorIHL)
        // }

        let header_length = usize::from(ihl) * 4;
        let dscp = u8::from_be_bytes([slice[1]]) >> 2;
        let ecn = u8::from_be_bytes([slice[1]]) & 0b0000_0011;
        let total_length = u16::from_be_bytes([slice[2], slice[3]]);
        let identification = u16::from_be_bytes([slice[4], slice[5]]);
        let dont_fragment = 0 != u8::from_be_bytes([slice[6]]) & 0x40;
        let more_fragment = 0 != u8::from_be_bytes([slice[6]]) & 0x20;
        let fragment_offset = u16::from_be_bytes([slice[6] & 0x1f, slice[7]]);
        let ttl = u8::from_be_bytes([slice[8]]);
        let protocol = u8::from_be_bytes([slice[9]]);
        let header_checksum = u16::from_be_bytes([slice[10], slice[11]]);
        let source_address = [slice[12], slice[13], slice[14], slice[15]];
        let dist_address = [slice[16], slice[17], slice[18], slice[19]];

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