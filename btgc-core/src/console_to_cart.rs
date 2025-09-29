use alloc::string::String;
use serde::{Deserialize, Serialize};

pub type SerialLinkHandShakeRes = Result<SerialLinkHandShakeOk, String>;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SerialLinkHandShakeOk {
    pub pin_out_mode: PinOutMode,
    pub speed: u32,
}
//     Err {
//         reason: String,
//     },
// }

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum PinOutMode {
    A,
    B,
}
