use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

use crate::{strategy_runner::BackgroundFn, stubs::DataPacket, Error};

pub(crate) enum BackgroundMessage {
    Data(DataPacket),
    HealthCheck,
}

pub(crate) struct BackgroundManager {
    thread_handle: JoinHandle<()>,
    sender: mpsc::Sender<BackgroundMessage>,
}

impl BackgroundManager {
    pub fn start(mut functions: Vec<Box<dyn BackgroundFn>>) -> Self {
        let (sender, receiver) = mpsc::channel();

        let thread_handle = thread::spawn(move || loop {
            let message = receiver.recv().unwrap();
            match &message {
                BackgroundMessage::Data(data) => {
                    for function in &mut functions {
                        // TODO: how should we handle errors.
                        function.process(data).unwrap();
                    }
                }
                BackgroundMessage::HealthCheck => {
                    println!("Background health check");
                }
            }
        });

        Self {
            thread_handle,
            sender,
        }
    }

    pub fn send(&self, message: BackgroundMessage) -> Result<(), Error> {
        self.sender.send(message).map_err(|_| todo!())
    }
}
