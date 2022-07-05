use crate::binance::BinanceExchange;
use crate::errors::*;
use crate::exchange::Exchange;
use crate::model::ExchangeType;

// 策略对象
pub struct Strategy {
    exchange: Option<Box<dyn Exchange>>, //交易所对象
}

impl Strategy {
    pub fn new() -> Strategy {
        Strategy { exchange: None }
    }

    pub fn init(&mut self, exchange_type: ExchangeType) -> Result<()> {
        match exchange_type {
            ExchangeType::Binance => {
                let mut exchange = BinanceExchange::new();
                exchange.init()?;
                self.exchange = Some(Box::new(exchange));
            }
        }
        Ok(())
    }
}
