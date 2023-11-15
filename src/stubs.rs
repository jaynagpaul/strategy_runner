use rand::Rng;
/// This file is used to stub out the functionality of the strategy runner
/// Before production this file should be removed

// pub type DataPacket = String;
pub struct DataPacket {
    pub data: DataEnum,
    pub exchange: String,
    pub channel: String,
}
pub enum DataEnum {
    M1(MessageType1),
    M2(MessageType2),
}
pub struct MessageType1 {
    pub data: String,
    pub best_ask: f64,
    pub ask_amt: f64,
}
pub struct MessageType2 {
    pub data: String,
    pub best_bid: f64,
    pub bid_amt: f64,
}

pub struct ExchangeListener {}

impl ExchangeListener {
    pub fn new() -> Self {
        Self {}
    }

    pub fn next(&mut self) -> Option<DataPacket> {
        let mut rng = rand::thread_rng();

        let x = rng.gen_bool(0.000001);

        if (!x) {
            return None;
        }

        let message_type1 = MessageType1 {
            data: "Example data".to_string(),
            best_ask: rng.gen_range(5.0..200.0),
            ask_amt: rng.gen_range(10.0..100.0),
        };

        // Use the instance in DataEnum
        let data_enum = DataEnum::M1(message_type1);

        // Create a DataPacket
        let data_packet = DataPacket {
            data: data_enum,
            exchange: "Binance".to_string(),
            channel: "Channel-1".to_string(),
        };

        Some(data_packet)
    }
}
