use crate::orderbook::Orderbook;

#[derive(Default)]
pub struct StrategyState {
    pub orderbook: Orderbook,
    pub huobi_btc_orderbook: Orderbook,
    pub huobi_eth_orderbook: Orderbook,
    pub binance_btc_orderbook: Orderbook,
    pub binance_eth_orderbook: Orderbook,
}
