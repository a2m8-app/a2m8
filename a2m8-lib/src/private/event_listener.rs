use std::thread;

use once_cell::sync::Lazy;
use rdev::{grab, listen, Event};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver},
    oneshot, Mutex,
};

pub static EVENT_LISTENER: Lazy<Mutex<UnboundedReceiver<Event>>> = Lazy::new(|| {
    let (schan, rchan) = mpsc::unbounded_channel();
    let _listener = thread::spawn(move || {
        listen(move |event| {
            schan
                .send(event)
                .unwrap_or_else(|e| println!("Could not send event {e:?}"));
        })
        .expect("Could not listen");
    });
    Mutex::new(rchan)
});
type LazyMutexUnboundReceiver<T> = Lazy<Mutex<UnboundedReceiver<T>>>;
pub static EVENT_GRABBER: LazyMutexUnboundReceiver<(Event, oneshot::Sender<Option<Event>>)> = Lazy::new(|| {
    let (schan, rchan) = mpsc::unbounded_channel();
    let _listener = thread::spawn(move || {
        grab(move |event| {
            let (response_sender, response_receiver) = oneshot::channel();
            schan
                .send((event, response_sender))
                .unwrap_or_else(|e| println!("Could not send event {e:?}"));
            response_receiver.blocking_recv().unwrap_or(None)
        })
        .expect("Could not listen");
    });
    Mutex::new(rchan)
});
