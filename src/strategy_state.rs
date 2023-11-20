use crate::orderbook::Orderbook;
use crate::binance_orderbook::BinanceOrderbook;
use crate::huobi_orderbook::HuobiOrderbook;

#[derive(Default)]
pub struct StrategyState {
    pub orderbook: Orderbook,
    pub huobi_orderbook: HuobiOrderbook,
    pub binance_orderbook: BinanceOrderbook,
}
