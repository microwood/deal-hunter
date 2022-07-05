use std::sync::atomic::AtomicBool;

use crate::errors::*;
use crate::exchange::Exchange;
use crate::tools::{WebSockets, WebsocketAPI, WebsocketEvent};

pub struct BinanceExchange<'a> {
    kline_socket: Option<WebSockets<'a>>,
}

impl<'a> BinanceExchange<'a> {
    pub fn new() -> BinanceExchange<'a> {
        BinanceExchange { kline_socket: None }
    }

    pub fn init(&mut self) -> Result<()> {
        self.init_kline_socket()
    }

    fn init_kline_socket(&mut self) -> Result<()> {
        let keep_running = AtomicBool::new(true);
        let kline_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
            if let WebsocketEvent::BinanceKline(kline_event) = event {
                println!(
                    "Symbol: {}, high: {}, low: {}",
                    kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
                );
            }
            Ok(())
        });
        self.kline_socket = Some(kline_socket);
        if let Some(websocket) = &mut self.kline_socket {
            websocket.connect(WebsocketAPI::Default.params("ethusdt@kline_1m"))?;
            if let Err(e) = websocket.event_loop(&keep_running) {
                println!("event loop is error: {:?}", e);
            };
            websocket.disconnect().unwrap();
        }
        Ok(())
    }
}

impl<'a> Exchange for BinanceExchange<'a> {}
