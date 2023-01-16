use std::thread;

use once_cell::sync::Lazy;
use rdev::{listen, Event};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver},
    Mutex,
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
