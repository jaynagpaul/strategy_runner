use rand::Rng;
pub use crate::stubs::ExchangeEnum::Huobi;
pub use crate::stubs::ExchangeEnum::Binance;
pub use crate::stubs::SymbolEnum::BTCUSD;
pub use crate::stubs::SymbolEnum::ETHUSD;

///TODO: trade detail
pub enum ExchangeEnum {
    Huobi, 
    Binance,
}

pub enum SymbolEnum {
    BTCUSD,
    ETHUSD,
}

pub struct DataPacket {
    pub data: DataEnum,
    pub exchange: ExchangeEnum,
    pub symbol_pair: SymbolEnum,
    pub channel: String,
    pub timestamp: i64,
}

pub enum DataEnum {
    MBP(MarketIncremental),
    RBA(RefreshBidAsk),
}

pub struct MarketIncremental {
    pub bestask: f64,
    pub askamount: f64,
    pub bestbid: f64,
    pub bidamount: f64,
}

pub struct RefreshBidAsk {
    pub asks: Vec<(f64, f64)>, //price, amount
    pub bids: Vec<(f64, f64)>, //price, amount
}

pub struct ExchangeListener {}

impl ExchangeListener {
    pub fn new() -> Self {
        Self {}
    }

    pub fn poll(&mut self) -> Option<DataPacket> {
        let mut rng = rand::thread_rng();

        let x = rng.gen_bool(0.000001);

        if !x {
            return None;
        }
        
        let tmp = rng.gen_range(5.0..200.0);

        let market_incremental = MarketIncremental {
            bestask: tmp,
            askamount: rng.gen_range(10.0..100.0),
            bestbid: rng.gen_range(5.0..tmp),
            bidamount: rng.gen_range(10.0..100.0)
        };

        // Use the instance in DataEnum
        let data_enum = DataEnum::MBP(market_incremental);

        // Create a DataPacket
        let data_packet = DataPacket {
            data: data_enum,
            exchange: Binance,
            symbol_pair: BTCUSD,
            channel: "Channel-1".to_string(),
            timestamp: 1
        };

        Some(data_packet)
    }
}
