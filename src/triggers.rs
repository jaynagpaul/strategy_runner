use enumset::{EnumSet, EnumSetType};

pub type Triggers = EnumSet<Event>;

#[derive(EnumSetType, Debug)]
pub enum Event {
    NewHighBid,
    NewLowAsk,
    NewBid,
    NewAsk,
    NewBinanceBTCHighBid,
    NewBinanceBTCLowAsk,
    NewBinanceBTCBid,
    NewBinanceBTCAsk,
    NewBinanceETHHighBid,
    NewBinanceETHLowAsk,
    NewBinanceETHBid,
    NewBinanceETHAsk,
    NewHuobiBTCHighBid,
    NewHuobiBTCLowAsk,
    NewHuobiBTCBid,
    NewHuobiBTCAsk,
    NewHuobiETHHighBid,
    NewHuobiETHLowAsk,
    NewHuobiETHBid,
    NewHuobiETHAsk,
}
