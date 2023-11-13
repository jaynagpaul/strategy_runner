use strategy_runner::{Error, Event, StrategyFn, StrategyRunnerBuilder, StrategyState, Triggers};

struct TestStrategy;

impl StrategyFn for TestStrategy {
    fn process(&mut self, triggers: Triggers, state: &StrategyState) -> Result<(), Error> {
        println!("TestStrategy::process");
        Ok(())
    }

    fn triggers(&self) -> Triggers {
        Event::BtcUpdate | Event::EthUpdate
    }
}

fn main() {
    let runner = StrategyRunnerBuilder::new()
        .add_exchange(todo!())
        .add_background_callback(todo!())
        .add_strategy(todo!())
        .build();

    runner.run().unwrap();
}
