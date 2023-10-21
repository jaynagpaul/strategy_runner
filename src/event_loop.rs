use crate::stubs::{DataPacket, ExchangeListener};

#[derive(Default)]
pub(crate) struct EventLoop {
    listeners: Vec<ExchangeListener>,
}

impl EventLoop {
    pub(crate) fn add_exchange(&mut self, exchange: ExchangeListener) {
        self.listeners.push(exchange);
    }

    pub(crate) fn poll(&mut self) -> DataPacket {
        if self.listeners.is_empty() {
            panic!("No exchanges registered");
        }

        loop {
            for listener in &mut self.listeners {
                match listener.next() {
                    None => continue,
                    Some(event) => {
                        return event;
                    }
                }
            }
        }
    }
}
