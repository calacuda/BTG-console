#![no_std]

use btgc_core::{
    Message, Result, SerialSendable, bail, cart_to_console::SerialLinkHandShakeReq,
    cart_to_term::InstructFrontEnd, console_to_cart::SerialLinkHandShakeRes,
};
use embedded_io::Write;
use embedded_io_async::Write as AsyncWrite;

pub trait GameCartInterface {
    /// initiallizes the Serial communicaitons between the console+cartridge and a
    /// console+cartridge linked byt a cable or other form of communicaitons.
    fn init_serial_link(&mut self, req_config: SerialLinkHandShakeReq) -> SerialLinkHandShakeRes;
    /// returns the uart device that comunicates with the terminal
    fn get_term_interface(&mut self) -> impl Write;
    /// sends a messasge to the font end client (the Term)
    fn instruct_term_to(&mut self, do_action: Message<InstructFrontEnd>) -> Result<()> {
        if let Err(e) = self.get_term_interface().write_all(&do_action.to_serial()?) {
            bail!("{e:?}")
        }

        Ok(())
    }
}

pub trait GameCartInterfaceAsyncExt: GameCartInterface {
    /// returns the uart device that comunicates with the terminal in an async fasion
    #[allow(async_fn_in_trait)]
    async fn async_get_term_interface(&mut self) -> impl AsyncWrite;
    /// sends a messasge to the font end client (the Term) in an async fasion
    #[allow(async_fn_in_trait)]
    async fn async_instruct_term_to(&mut self, do_action: Message<InstructFrontEnd>) -> Result<()> {
        if let Err(e) = self
            .async_get_term_interface()
            .await
            .write_all(&do_action.to_serial()?)
            .await
        {
            bail!("{e:?}")
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
