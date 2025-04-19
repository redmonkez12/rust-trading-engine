mod matching_engine;

use std::collections::HashMap;
use matching_engine::orderbook::{Order, BidOrAsk, OrderBook};
use matching_engine::engine::{MatchingEngine, TradingPair};

fn main() {
    let mut order_book = OrderBook::new();

    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    order_book.add_order(4.4, buy_order_from_alice);
    order_book.add_order(4.4, buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    order_book.add_order(20.0, sell_order);

    // println!("Price {:?}", order_book);

    let mut engine = MatchingEngine::new();
    let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    engine.add_new_market(eth_pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    engine.place_limit_order(eth_pair, 10.000, buy_order).unwrap()

}
