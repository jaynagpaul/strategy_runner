/// This file is used to stub out the functionality of the strategy runner
/// Before production this file should be removed

// pub type DataPacket = String;
pub struct DataPacket {
    pub Data: DataEnum,
    pub Exchange: String,
    pub Channel: String
}
pub enum DataEnum {
    M1(MessageType1),
    M2(MessageType2),
}
pub struct MessageType1{
    pub data: String,
    pub BestAsk: f64,
    pub AskAmt: f64,
}
pub struct MessageType2{
    pub data: String,
    pub BestBid: f64,
    pub BidAmt: f64,
}

pub struct ExchangeListener {}

impl ExchangeListener {
    pub fn new() -> Self {
        Self {}
    }

    pub fn next(&mut self) -> Option<DataPacket> {
        Some(DataPacket::new())
    }
}
