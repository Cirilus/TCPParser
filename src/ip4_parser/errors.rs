use thiserror::Error;

#[derive(Error, Debug)]
pub enum IP4Errors {
    #[error("the min len of the ip4 packet is 20")]
    ErrMinLenPacket(),

    #[error("error version of the ip4")]
    ErrVersion(),

    #[error("error ihl of the ip4")]
    ErrorIHL(),
}