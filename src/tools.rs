use crate::errors::*;
use crate::model::BinanceKlineEvent;

use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::handshake::client::Response;
use tungstenite::protocol::WebSocket;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message};
use url::Url;

pub struct WebSockets<'a> {
    pub socket: Option<(WebSocket<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(WebsocketEvent) -> Result<()> + 'a>,
}

impl<'a> WebSockets<'a> {
    pub fn new<Callback>(handler: Callback) -> WebSockets<'a>
    where
        Callback: FnMut(WebsocketEvent) -> Result<()> + 'a,
    {
        println!("this is websocket new function");
        WebSockets {
            socket: None,
            handler: Box::new(handler),
        }
    }

    pub fn connect(&mut self, path: String) -> Result<()> {
        let url = Url::parse(&path)?;
        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => bail!(format!("Error during handshake {}", e)),
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            return Ok(());
        }
        bail!("Not able to close the connection");
    }

    pub fn handle_msg(&mut self, msg: &str) -> Result<()> {
        let value: serde_json::Value = serde_json::from_str(msg)?;

        if let Some(data) = value.get("data") {
            self.handle_msg(&data.to_string())?;
            return Ok(());
        }

        if let Ok(events) = serde_json::from_value::<Events>(value) {
            let action = match events {
                Events::KlineEvent(v) => WebsocketEvent::BinanceKline(v),
            };
            (self.handler)(action)?;
        }
        Ok(())
    }

    pub fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some(ref mut socket) = self.socket {
                let message = socket.0.read_message()?;
                match message {
                    Message::Text(msg) => {
                        if let Err(e) = self.handle_msg(&msg) {
                            bail!(format!("Error on handling stream message: {}", e));
                        }
                    }
                    Message::Ping(_)
                    | Message::Pong(_)
                    | Message::Binary(_)
                    | Message::Frame(_) => (),
                    Message::Close(e) => bail!(format!("Disconnected {:?}", e)),
                }
            }
        }
        Ok(())
    }
}

#[allow(clippy::all)]
pub enum WebsocketAPI {
    Default,
    Custom(String),
}

impl WebsocketAPI {
    pub fn params(self, subscription: &str) -> String {
        match self {
            WebsocketAPI::Default => format!("wss://fstream.binance.com/ws/{}", subscription),
            WebsocketAPI::Custom(url) => format!("{}/{}", url, subscription),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebsocketEvent {
    BinanceKline(BinanceKlineEvent),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Events {
    KlineEvent(BinanceKlineEvent),
}
