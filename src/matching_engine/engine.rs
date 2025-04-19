use std::collections::HashMap;
use super::orderbook::{Order, OrderBook};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String, // BTC
    quote: String, // USD
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> Self {
        TradingPair {
            base,
            quote,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>
}

impl MatchingEngine {
    pub fn new() -> Self {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        println!("New market added: {:?}", pair.to_string());
    }

    pub fn place_limit_order(&mut self, pair: TradingPair, price: f64, order: Order) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbooks) => {
                orderbooks.add_order(price, order);

                println!("placed limit order at price level: {}", price);

                Ok(())
            },
            None => {
                Err(format!("the orderbook for the given trading pair ({}) does not exist:", pair.to_string()))
            }
        }
    }
}
