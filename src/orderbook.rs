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

    pub fn update_bid(&mut self, best_bid: f64, bid_amt: f64) -> EnumSet<Event> {
        let mut triggers = EnumSet::new();
        let best_bid = OrderedFloat(best_bid);

        if let Some(highest_bid) = self.bids.keys().next_back() {
            if &best_bid > highest_bid {
                triggers.insert(Event::NewHighBid);
            }
            // if &best_bid == highest_bid {
            //     triggers.insert(Event::NewLowBid);
            // }
        }
        if bid_amt == 0 {
            //     I don't believe it is possible to delete from a treemap in logarithmic time... may need to code custom btreemap
        }
        self.bids.insert(best_bid, bid_amt);
        triggers.insert(Event::NewBid);

        triggers
    }

    pub fn update_ask(&mut self, best_ask: f64, ask_amt: f64) -> EnumSet<Event> {
        let mut triggers = EnumSet::new();
        let best_ask = OrderedFloat(best_ask);

        if let Some(lowest_ask) = self.asks.keys().next() {
            if &best_ask < lowest_ask {
                triggers.insert(Event::NewLowAsk);
            }
        } else {
            triggers.insert(Event::NewLowAsk);
        }

        self.asks.insert(best_ask, ask_amt);
        triggers.insert(Event::NewAsk);

        triggers
    }

    pub fn update_market_incremental(&mut self, timestamp: i64, asks: Vec<(f64, f64)>, bids: Vec<(f64, f64)>) -> EnumSet<Event>{
        let mut triggers = EnumSet::new();
        if start_time == last_timestamp + 1 {
            self.last_timestamp = timestamp;
            for dp in asks.into_iter() { triggers = triggers | self.update_ask(dp.0, dp.1); }
            for dp in bids.into_iter() { triggers = triggers | self.update_bid(dp.0, dp.1); }
        } 
        else if start_time > last_timestamp + 1 {

        }
        triggers
    }

    pub fn update_refresh(&mut self, timestamp: i64, asks: Vec<(f64, f64)>, bids: Vec<(f64, f64)>) -> EnumSet<Event>{

    }
}

impl DataStructure for Orderbook {
    fn update(&mut self, dp : &DataPacket) -> EnumSet<Event> {
        let mut triggers = EnumSet::new();
        if let DataEnum::MBP(msg) = &dp.data {
            // Determine if I need to add to the queue here?

            triggers = triggers | self.update_bid(msg.bestbid, msg.bidamount);
            triggers = triggers | self.update_ask(msg.bestask, msg.askamount);
        }
        triggers
    }
}

// TODO:
// - Need to discuss what datapackets will look like exactly
// - BTreeMap doesn't allow for easy deletion
// - Is the queue necessary?
// - Should we use generic DataStructure trait?