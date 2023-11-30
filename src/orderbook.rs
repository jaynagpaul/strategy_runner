use crate::Event;
use enumset::EnumSet;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use crate::data_structure::DataStructure;
use crate::stubs::*;

#[derive(Default)]
pub struct Orderbook {
    bids: BTreeMap<OrderedFloat<f64>, f64>,
    asks: BTreeMap<OrderedFloat<f64>, f64>,
    awaiting_refresh: bool, // initially false
    last_timestamp: i64, // initially -1
    awaiting_packets: Queue<(DataPacket)>,
}

impl Orderbook {
    pub fn lowest_ask(&self) -> Option<f64> {
        self.asks.keys().next().map(|x| x.into_inner())
    }
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.keys().next_back().map(|x| x.into_inner())
    }

    pub fn update_bid(&mut self, bid_val: f64, bid_amt: f64) -> EnumSet<Event> {
        let mut triggers = EnumSet::new();
        let bid_val = OrderedFloat(best_bid);

        if let Some(highest_bid) = self.bids.keys().next_back() {
            if &bid_val > highest_bid {
                triggers.insert(Event::NewHighBid);
            }
        }
        if bid_amt == 0 {
            self.bids.remove(bid_val);
        } else {
            if !self.bids.contains_key(bid_val) {
                triggers.insert(Event::NewBid);
            }
            self.bids.insert(bid_val, bid_amt);
        }

        triggers
    }

    pub fn update_ask(&mut self, ask_val: f64, ask_amt: f64) -> EnumSet<Event> {
        let mut triggers = EnumSet::new();
        let ask_val = OrderedFloat(ask_val);

        if let Some(lowest_ask) = self.asks.keys().next() {
            if &ask_val < lowest_ask {
                triggers.insert(Event::NewLowAsk);
            }
        } else {
            triggers.insert(Event::NewLowAsk);
        }

        if ask_amt == 0 {
            self.asks.remove(ask_val);
        } else {
            if !self.asks.contains_key(ask_val) {
                triggers.insert(Event::NewAsk);
            }
            self.asks.insert(ask_val, ask_amt);
        }

        triggers
    }

    pub fn update_market_incremental(&mut self, asks: Vec<(f64, f64)>, bids: Vec<(f64, f64)>) -> EnumSet<Event>{
        let mut triggers = EnumSet::new();
        for dp in asks.into_iter() { triggers = triggers | self.update_ask(dp.0, dp.1); }
        for dp in bids.into_iter() { triggers = triggers | self.update_bid(dp.0, dp.1); }
        triggers
    }

    pub fn update_refresh(&mut self, timestamp: i64, asks: Vec<(f64, f64)>, bids: Vec<(f64, f64)>) -> EnumSet<Event>{
        let mut triggers = EnumSet::new();

        triggers
    }
}

impl DataStructure for Orderbook {
    fn update(&mut self, dp : &DataPacket) -> EnumSet<Event> {
        let mut triggers = EnumSet::new();
        // match &dp.data {
        //     DataEnum::MBP(msg)
        //     DataEnum::RBA(msg)
        // }
        if let DataEnum::MBP(msg) = &dp.data {

            // Determine if I need to add to the queue here?
            
            if dp.prev_timestamp > last_timestamp {
                awaiting_refresh = true;
                awaiting_packets.add(dp);
            }
            update_market_incremental(msg.asks, msg.bids);
        }
        triggers
    }
}

// TODO:
// - Need to discuss what datapackets will look like exactly
// - BTreeMap doesn't allow for easy deletion
// - Is the queue necessary?
// - Should we use generic DataStructure trait?