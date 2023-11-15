use strategy_runner::{
    Error, Event, ExchangeListener, StrategyFn, StrategyRunnerBuilder, StrategyState, Triggers,
};

struct TestStrategy(u32);

impl StrategyFn for TestStrategy {
    fn process(&mut self, triggers: Triggers, state: &StrategyState) -> Result<(), Error> {
        self.0 += 1;
        println!("Strategy has run {} times", self.0);

        println!("New Low Ask {}", state.orderbook.lowest_ask().unwrap());
        Ok(())
    }

    fn triggers(&self) -> Triggers {
        Event::NewLowAsk | Event::NewHighBid
    }
}

fn main() {
    let mut builder = StrategyRunnerBuilder::new();
    builder
        .add_exchange(ExchangeListener {})
        .add_strategy(Box::new(TestStrategy(0)));

    let mut runner = builder.build();

    runner.run().unwrap();
}
