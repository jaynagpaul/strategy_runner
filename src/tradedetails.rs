use crate::Event;
use enumset::EnumSet;
use ordered_float::OrderedFloat;
use std::collections::Queue;

#[derive(Default)]
pub struct Trade {
    amount : f64,
    price: f64,
    direction : bool, // should probably change to string
}

#[derive(Default)]
pub struct TradeDetails {
    buy_details : Queue<Trade>,
    sell_detials: Queue<Trade>,
}

impl TradeDetails {
    pub fn lowest_ask(&self) -> Option<f64> {
        self.asks.keys().next().map(|x| x.into_inner())
    }
    pub fn add_trade(&mut self, amount: f64, price: f64, direction: bool) -> EnumSet<Event>{
        let mut triggers = EnumSet::new();

        if bool {
            self.buy_details.add(Trade{amount, price, direction});
            triggers.insert(Event::NewBuyTrade);
        } else {
            self.sell_details.add(Trade{amount, price, direction});
            triggers.insert(Event::NewSellTrade);
        }

        triggers
    }

    pub fn pop_trade(&mut self, direction : bool) -> Option<Trade> {
        if direction {

        } else {

        }
    }
}
