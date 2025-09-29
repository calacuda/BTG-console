use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SerialLinkHandShakeReq {
    pub uuid: [u8; 32],
    pub sugested_speed: u32,
}
