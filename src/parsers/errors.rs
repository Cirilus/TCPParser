use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("ethernet error: {0}")]
    EthernetError(EthernetError),

    #[error("ip4 error: {0}")]
    IP4Error(IP4Error),

    #[error("tcp error: {0}")]
    TCPError(TCPError),
}


impl From<EthernetError> for PacketError {
    fn from(value: EthernetError) -> Self {
        return PacketError::EthernetError(value)
    }
}

impl From<IP4Error> for PacketError {
    fn from(value: IP4Error) -> Self {
        return PacketError::IP4Error(value)
    }
}

impl From<TCPError> for PacketError {
    fn from(value: TCPError) -> Self {
        return PacketError::TCPError(value)
    }
}

#[derive(Error, Debug)]
pub enum EthernetError {

}

#[derive(Error, Debug)]
pub enum IP4Error {
    #[error("the min len of the ip4 packet must be at least 20")]
    ErrMinLenPacket,

    #[error("invalid version")]
    ErrVersion,

    #[error("invalid ihl")]
    ErrorIHL,
}

#[derive(Error, Debug)]
pub enum TCPError {
}

