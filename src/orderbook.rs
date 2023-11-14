use crate::Event;
use enumset::EnumSet;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

pub struct Orderbook {
    bids: BTreeMap<OrderedFloat<f64>, f64>,
    asks: BTreeMap<OrderedFloat<f64>, f64>,
}

impl Orderbook {
    pub fn update_bid(&mut self, best_bid: f64, bid_amt: f64) -> EnumSet<Event> {
        let mut triggers = EnumSet::new();
        let best_bid = OrderedFloat(best_bid);

        self.bids.insert(best_bid, bid_amt);
        if let Some(highest_bid) = self.bids.keys().next_back() {
            if &best_bid > highest_bid {
                triggers.insert(Event::NewHighBid);
            }
        }

        triggers
    }

    pub fn update_ask(&mut self, best_ask: f64, ask_amt: f64) -> EnumSet<Event> {
        let mut triggers = EnumSet::new();
        let best_ask = OrderedFloat(best_ask);

        self.asks.insert(best_ask, ask_amt);
        if best_ask < *self.asks.keys().next().unwrap_or(&OrderedFloat(f64::MAX)) {
            triggers.insert(Event::NewLowAsk);
        }

        triggers
    }
}
