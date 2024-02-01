use crate::parsers::errors::PacketError;
use crate::parsers::types::{Ethernet, IP4, Packet, TCP};

impl Packet {
    pub fn from_slice(slice: &[u8]) -> Result<Packet, PacketError>{
        let eth = Ethernet::from_slice(&slice[..5])?;
        let ip4 = IP4::from_slice(&slice[4..25])?;
        let tcp = TCP::from_slice(&slice[24..])?;

        Ok(Packet{
            eth,
            ip4,
            tcp,
        })
    }
}