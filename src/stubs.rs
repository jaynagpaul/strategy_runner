use crate::{Error, StrategyState};

/// This file is used to stub out the functionality of the strategy runner
/// Before production this file should be removed

pub type DataPacket = String;

pub struct ExchangeListener {}

impl ExchangeListener {
    pub fn next(&mut self) -> Option<DataPacket> {
        unimplemented!()
    }
}

pub type Orderbook = ();

// TODO: these aren't stubs and should be defined here
pub trait StrategyFn {
    fn process(&mut self, data: &DataPacket, state: &mut StrategyState) -> Result<(), Error>;
}

pub trait BackgroundFn {
    fn process(&mut self, data: DataPacket) -> Result<(), Error>;
}
