use btgc_core::{
    cart_to_term::InstructFrontEnd, term_to_cart::UpdateFromClient, Message, SerialSendable,
};
use btgc_term_lib::{FxHashMap, ProcessedMessageRes, TerminalAPI};
use dioxus::prelude::*;
use serialport::available_ports;
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

fn recv_mesg_loop(api: Arc<RwLock<TerminalAPI>>) -> ! {
    // init serial interface
    let mut ports = FxHashMap::default();

    loop {
        if let Ok(available_ports) = available_ports() {
            for port in available_ports {
                if !ports.contains_key(&port.port_name) {
                    let Ok(ser_port) = serialport::new(port.port_name.clone(), 115200)
                        .timeout(Duration::from_millis(10))
                        .open()
                    else {
                        continue;
                    };
                    ports.insert(port.port_name, ser_port);
                }
            }
        }

        for (name, port) in &mut ports {
            let mut buf = Vec::with_capacity(1024);

            // recv message
            port.read_to_end(&mut buf);

            // do message thing
            let Ok(message) = Message::<InstructFrontEnd>::from_serial(&buf) else {
                continue;
            };

            if let Ok(ref mut api) = api.write() {
                let Ok(message) = api.process_msg(message) else {
                    continue;
                };

                match message {
                    ProcessedMessageRes::SendAck(id) => {
                        let msg = Message::<UpdateFromClient>::Ack { mesg: id };

                        if let Ok(msg_bytes) = msg.to_serial() {
                            _ = port.write_all(&msg_bytes);
                        }
                    }
                    ProcessedMessageRes::Resend(id) => {
                        // TODO:: resend message
                    }
                    ProcessedMessageRes::Success => {}
                    ProcessedMessageRes::TermMustHandle(message) => match message {
                        InstructFrontEnd::DisplayTiles { tiles } => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                            for (location, id) in tiles {
                                // TODO: set the proper displaying tile to teh image nessesary
                            }
                        }
                        InstructFrontEnd::DespawnSprite {
                            location: _,
                            tile_id: _,
                        } => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                        }
                        InstructFrontEnd::DisplaySprite {
                            location: _,
                            tile_id: _,
                        } => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                        }
                        InstructFrontEnd::MoveSprite {
                            from: _,
                            tile_id: _,
                            to: _,
                        } => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                        }
                        InstructFrontEnd::DisableControllerInput { id: _ } => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                        }
                        InstructFrontEnd::DisplayLoadingScreen => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                        }
                        InstructFrontEnd::DisplaySprites { sprites: _ } => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                        }
                        InstructFrontEnd::LoadMap { map: _ } => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                        }
                        InstructFrontEnd::LoadAsset {
                            uid: _,
                            asset_type: _,
                            data: _,
                        } => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                        }
                        InstructFrontEnd::DisableKeyboardInput { id: _ } => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                        }
                        InstructFrontEnd::DisableMouseInput { id: _ } => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                        }
                        InstructFrontEnd::DisplayTile {
                            location: _,
                            tile_id: _,
                        } => {
                            // return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    let api = Arc::new(RwLock::new(TerminalAPI::default()));

    let _jh = std::thread::spawn(move || recv_mesg_loop(api));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
