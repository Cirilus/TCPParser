use crate::parsers::errors::EthernetError;
use crate::parsers::types::Ethernet;

impl Ethernet {
    pub fn from_slice(slice: &[u8]) -> Result<Ethernet, EthernetError> {
        let flags = u16::from_be_bytes([slice[0], slice[1]]);
        let protocol = u16::from_be_bytes([slice[2], slice[3]]);
        Ok(Ethernet{
            flags,
            protocol,
        })
    }
}