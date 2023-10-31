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

#[test]
fn test_basic() {
    let exchange_listener = ExchangeListener::new();

    let mut event_loop = EventLoop::default();

    event_loop.add_exchange(exchange_listener);

    let event = event_loop.poll();

    assert_eq!(event, DataPacket::new());
}
