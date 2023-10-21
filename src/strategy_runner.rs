use std::{sync::mpsc, thread};

use crate::{
    background_manager::{BackgroundManager, BackgroundMessage},
    event_loop::{self, EventLoop},
    stubs::DataPacket,
    Error, ExchangeListener, StrategyState,
};

#[derive(Default)]
pub struct StrategyRunnerBuilder {
    event_loop: EventLoop,

    strategies: Vec<Box<dyn StrategyFn>>,
    background_callbacks: Vec<Box<dyn BackgroundFn>>,
}

pub struct StrategyRunner {
    event_loop: EventLoop,
    state: StrategyState,

    strategies: Vec<Box<dyn StrategyFn>>,
    background_manager: BackgroundManager,
}

pub trait StrategyFn {
    fn process(&mut self, data: &DataPacket, state: &mut StrategyState) -> Result<(), Error>;
}

pub trait BackgroundFn: Send {
    fn process(&mut self, data: &DataPacket) -> Result<(), Error>;
}

impl StrategyRunner {
    pub(crate) fn new(builder: StrategyRunnerBuilder) -> Self {
        Self {
            event_loop: builder.event_loop,
            state: StrategyState::default(),

            strategies: builder.strategies,
            background_manager: BackgroundManager::start(builder.background_callbacks),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            let data = self.event_loop.poll();
            self.dispatch(data)?;
        }
    }

    fn dispatch(&mut self, data: DataPacket) -> Result<(), Error> {
        for strategy in &mut self.strategies {
            strategy.process(&data, &mut self.state)?;
        }

        self.send_to_worker(data)
    }

    fn send_to_worker(&self, data: DataPacket) -> Result<(), Error> {
        self.background_manager.send(BackgroundMessage::Data(data))
    }
}

impl StrategyRunnerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_exchange(&mut self, exchange: ExchangeListener) -> &mut Self {
        self.event_loop.add_exchange(exchange);
        self
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn StrategyFn>) -> &mut Self {
        self.strategies.push(strategy);
        self
    }

    pub fn add_background_callback(&mut self, callback: Box<dyn BackgroundFn>) -> &mut Self {
        self.background_callbacks.push(callback);
        self
    }

    pub fn build(self) -> StrategyRunner {
        StrategyRunner::new(self)
    }
}
