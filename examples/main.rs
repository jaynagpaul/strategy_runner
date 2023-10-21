use strategy_runner::StrategyRunnerBuilder;
fn main() {
    let runner = StrategyRunnerBuilder::new()
        .add_exchange(todo!())
        .add_background_callback(todo!())
        .add_strategy(todo!())
        .build();

    runner.run().unwrap();
}
