use btgc_term_lib::{FxHashMap, TerminalAPI};
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
                    let ser_port = serialport::new(port.port_name.clone(), 115200)
                        .timeout(Duration::from_millis(10))
                        .open();
                    ports.insert(port.port_name, ser_port);
                }
            }
        }

        for (name, port) in &mut ports {
            // TODO: recv message

            // TODO: do message thing
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
