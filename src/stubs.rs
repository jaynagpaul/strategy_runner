use crate::{Error, StrategyState};

/// This file is used to stub out the functionality of the strategy runner
/// Before production this file should be removed

pub type DataPacket = String;

pub struct ExchangeListener {}

impl ExchangeListener {
    pub fn next(&mut self) -> Option<DataPacket> {
        Some(DataPacket::new())
    }
}

pub type Orderbook = ();
