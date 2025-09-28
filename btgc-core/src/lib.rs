#![no_std]

extern crate alloc;

pub mod cart_to_console;
pub mod cart_to_term;
pub mod console_to_cart;
pub mod term_to_cart;

pub type Result<T> = anyhow::Result<T>;
pub type MessageId = u32;
pub type AssetId = u32;

pub trait SerialSend: Sized {
    /// converts this type to bytes to be sendable over serial.
    fn to_serial(&self) -> impl Iterator<Item = u8>;
    /// creates this type from serial data.
    fn from_serial(data: impl Iterator<Item = u8>) -> Result<Self>;
}

#[derive(alloc::fmt::Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Message<T>
where
    T: alloc::fmt::Debug
        + Clone
        + Copy
        + core::hash::Hash
        + Eq
        + PartialEq
        + PartialOrd
        + Ord
        + SerialSend,
{
    Ack {
        /// the message being acknowledged as being successfully sent.
        mesg: MessageId,
        // ///
        // success: bool,
    },
    ResendPls {
        /// the message that needs resending.
        mesg: MessageId,
    },
    New {
        /// the message being transmitted
        mesg: T,
        /// check sum of the above message AFTER being SerialSend.to_serial
        checksum: [u8; 32],
    },
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
