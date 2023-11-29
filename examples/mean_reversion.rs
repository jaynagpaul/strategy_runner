use strategy_runner::{
    Error, Event, ExchangeListener, StrategyFn, StrategyRunnerBuilder, StrategyState, Triggers,
};

struct MeanReversionStrategy {
    moving_average: f64,
    num_samples: usize,
}


impl StrategyFn for MeanReversionStrategy {
    fn process(&mut self, triggers: Triggers, state: &StrategyState) -> Result<(), Error> {

        if triggers.contains(Event::NewBinanceBTCAsk) {
            let ask = state.get("binance_btc_ask")?;
            self.moving_average = (self.moving_average * self.num_samples as f64 + ask) / (self.num_samples + 1) as f64;
            self.num_samples += 1;
        }

        if triggers.contains(Event::NewBinanceBTCBid) {
            let bid = state.get("binance_btc_bid")?;
            self.moving_average = (self.moving_average * self.num_samples as f64 + bid) / (self.num_samples + 1) as f64;
            self.num_samples += 1;
        }

        Ok(())
        
    }

    fn triggers(&self) -> Triggers {
        Event::NewBinanceBTCAsk | Event::NewBinanceBtcBid
    }
}

fn main() {
    let mut builder = StrategyRunnerBuilder::new();
    builder
        .add_exchange(ExchangeListener {})
        .add_strategy(Box::new(MeanReversionStrategy);

    let mut runner = builder.build();

    runner.run().unwrap();
}
