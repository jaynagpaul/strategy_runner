use crate::{
    background_manager::{BackgroundManager, BackgroundMessage},
    event_loop::EventLoop,
    stubs::*,
    Error, ExchangeListener, StrategyState, Triggers,
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
    fn process(&mut self, triggers: Triggers, state: &StrategyState) -> Result<(), Error>;

    fn triggers(&self) -> Triggers;
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
        let triggers = self.update_state(&data)?;

        for strategy in &mut self.strategies {
            if !(triggers & strategy.triggers()).is_empty() {
                strategy.process(triggers, &self.state)?;
            }
        }

        self.send_to_worker(data)
    }

    fn update_state(&mut self, data: &DataPacket) -> Result<Triggers, Error> {
        match &data.exchange {
            ExchangeEnum::Huobi => {
                match &data.symbol_pair {
                    SymbolEnum::BTCUSD => {
                        match &data.data {
                            DataEnum::MBP(msg) => {
                                let triggers = self.state.huobi_btc_orderbook.update_ask(msg.bestask, msg.askamount) 
                                    | self.state.huobi_btc_orderbook.update_bid(msg.bestbid, msg.bidamount);
                                Ok(triggers)
                            }
                        }
                    }
                    SymbolEnum::ETHUSD => {
                        match &data.data {
                            DataEnum::MBP(msg) => {
                                let triggers = self.state.huobi_eth_orderbook.update_ask(msg.bestask, msg.askamount) 
                                    | self.state.huobi_eth_orderbook.update_bid(msg.bestbid, msg.bidamount);
                                Ok(triggers)
                            }
                        }
                    }
                }
            }
            ExchangeEnum::Binance => {
                match &data.symbol_pair {
                    SymbolEnum::BTCUSD => {
                        match &data.data {
                            DataEnum::MBP(msg) => {
                                let triggers = self.state.binance_btc_orderbook.update_ask(msg.bestask, msg.askamount) 
                                    | self.state.binance_btc_orderbook.update_bid(msg.bestbid, msg.bidamount);
                                Ok(triggers)
                            }
                        }
                    }
                    SymbolEnum::ETHUSD => {
                        match &data.data {
                            DataEnum::MBP(msg) => {
                                let triggers = self.state.binance_eth_orderbook.update_ask(msg.bestask, msg.askamount) 
                                    | self.state.binance_eth_orderbook.update_bid(msg.bestbid, msg.bidamount);
                                Ok(triggers)
                            }
                        }
                    }   
                }
            }
            // DataEnum::RBA(msg) => {
            //     let triggers = 
            // }
            // todo: new match shit


            // DataEnum::M1(msg) => {
            //     let triggers = self.state.orderbook.update_ask(msg.best_ask, msg.ask_amt);
            //     Ok(triggers)
            // }
            // DataEnum::M2(msg) => {
            //     let triggers = self.state.orderbook.update_bid(msg.best_bid, msg.bid_amt);
            //     Ok(triggers)
            // }
        }
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
