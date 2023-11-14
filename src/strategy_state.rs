use crate::orderbook::Orderbook;

#[derive(Default)]
pub struct StrategyState {
    pub orderbook: Orderbook,
}
