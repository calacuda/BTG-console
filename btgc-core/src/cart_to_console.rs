#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct SerialLinkHandShakeReq {
    pub uuid: [u8; 32],
    pub sugested_speed: u32,
}
