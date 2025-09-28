use alloc::string::String;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum SerialLinkHandShakeRes {
    Ok {
        pin_out_mode: PinOutMode,
        speed: u32,
    },
    Err {
        reason: String,
    },
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum PinOutMode {
    A,
    B,
}
