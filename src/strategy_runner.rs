use std::{sync::mpsc, thread};

use crate::{
    background_manager::BackgroundManager,
    event_loop::EventLoop,
    stubs::{DataPacket, StrategyFn},
    BackgroundFn, Error, ExchangeListener, StrategyState,
};

pub struct StrategyRunner {
    event_loop: EventLoop,
    state: StrategyState,

    strategies: Vec<Box<dyn StrategyFn>>,
    background_manager: BackgroundManager,
}

impl StrategyRunner {
    pub fn run(&mut self) -> Result<(), Error> {
        self.setup_background_worker();

        loop {
            let data = self.event_loop.poll();
            self.dispatch(data)?;
        }
    }

    fn setup_background_worker(&mut self) {
        // let (rx, tx) = mpsc::channel();

        thread::spawn(move || todo!());
    }

    fn dispatch(&mut self, data: DataPacket) -> Result<(), Error> {
        for strategy in &mut self.strategies {
            strategy.process(&data, &mut self.state)?;
        }

        //this.copy_to_background_worker(data);

        Ok(())
    }

    pub fn register_exchange(&mut self, exchange: ExchangeListener) -> Result<(), Error> {
        todo!()
    }

    pub fn register_strategy(&mut self, strategy: Box<dyn StrategyFn>) -> Result<(), Error> {
        self.strategies.push(strategy);

        Ok(())
    }

    pub fn register_background_callback(
        &mut self,
        callback: impl BackgroundFn,
    ) -> Result<(), Error> {
        todo!()
    }
}
