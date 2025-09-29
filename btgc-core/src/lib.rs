#![no_std]

use alloc::vec::Vec;
pub use anyhow::bail;
use bincode::{
    config::Configuration,
    serde::{decode_from_slice, encode_to_vec},
};
use serde::{Deserialize, Serialize};

extern crate alloc;

pub mod cart_to_console;
pub mod cart_to_term;
pub mod console_to_cart;
pub mod term_to_cart;

pub type Result<T> = anyhow::Result<T>;
pub type MessageId = u32;
pub type AssetId = u32;

pub trait SerialSendable: Sized {
    /// converts this type to bytes to be sendable over serial.
    fn to_serial(&self) -> Result<Vec<u8>>;
    /// creates this type from serial data.
    fn from_serial(data: &[u8]) -> Result<Self>;
    /// the to config used for "bytification"
    fn get_config() -> Configuration {
        bincode::config::standard().with_variable_int_encoding()
    }
}

#[derive(
    alloc::fmt::Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub enum Message<T>
where
    T: alloc::fmt::Debug + Clone + core::hash::Hash + Eq + PartialEq + PartialOrd + Ord, // + Serialize, // + for<'b> Deserialize<'b>,
                                                                                         // + Deserialize,
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
        /// whether an ACK mesasge should be sent or not
        ack_required: bool,
    },
}

impl<T> SerialSendable for Message<T>
where
    T: alloc::fmt::Debug
        + Clone
        // + Copy
        + core::hash::Hash
        + Eq
        + PartialEq
        + PartialOrd
        + Ord
        + Serialize // + Deserialize,
        + for<'a> Deserialize<'a>,
{
    fn to_serial(&self) -> Result<Vec<u8>> {
        Ok(encode_to_vec(self, Self::get_config())?)
    }

    fn from_serial(data: &[u8]) -> Result<Self> {
        Ok(decode_from_slice(data, Self::get_config())?.0)
    }
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
