use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

use crate::{
    stubs::{BackgroundFn, DataPacket},
    Error,
};

pub(crate) enum BackgroundMessage {
    Data(DataPacket),
    HealthCheck,
}

pub(crate) struct BackgroundManager {
    // thread_handle: JoinHandle,
    sender: mpsc::Sender<BackgroundMessage>,
}

impl BackgroundManager {
    fn start(mut functions: Vec<Box<dyn BackgroundFn>>) -> Self {
        let (sender, receiver) = mpsc::channel();

        let thread_handle = thread::spawn(move || loop {
            let message = receiver.recv().unwrap();
            match &message {
                BackgroundMessage::Data(data) => {
                    for function in &mut functions {
                        function.process(data).unwrap();
                        todo!()
                    }
                }
                BackgroundMessage::HealthCheck => {}
            }
        });

        Self { sender }
    }

    fn send(&self, message: BackgroundMessage) -> Result<(), Error> {
        self.sender.send(message).map_err(|_| todo!())
    }
}
