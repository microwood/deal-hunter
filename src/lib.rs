#[macro_use]
extern crate error_chain;

mod binance;
mod errors;
mod exchange;
mod model;
mod strategy;
mod tools;

use model::ExchangeType;
use strategy::Strategy;

pub fn launch() {
    let exchange_type = 0u8;
    let exchange_type = ExchangeType::try_from(exchange_type).unwrap();
    let mut strategy = Strategy::new();
    if let Err(e) = strategy.init(exchange_type) {
        println!("Failed to initialize strategy: {}", e);
    };
}
