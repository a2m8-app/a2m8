use std::thread;

use clipboard::Clipboard;
use displays::EasyDisplay;
use event_handler::EventHandler;
use mlua::{Error as LuaError, Lua};
use once_cell::sync::Lazy;
use rdev::{listen, Event};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver},
    Mutex,
};

mod clipboard;
mod displays;
mod event_handler;

static EVENT_LISTENER: Lazy<Mutex<UnboundedReceiver<Event>>> = Lazy::new(|| {
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

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), LuaError> {
    let lua = Lua::new();
    // let screenshot = lua.create_function(|_, ()| -> Result<(), LuaError> {
    //     return 1;
    // })?;
    // lua.globals().set("rust_func", f)?;
    lua.globals().set("display", EasyDisplay {})?;
    lua.globals().set("event_handler", EventHandler {})?;
    lua.globals().set("clipboard", Clipboard {})?;
    std::env::set_current_dir("./src").unwrap();
    lua.load(&std::fs::read_to_string("script.lua")?).exec_async().await?;
    Ok(())
}
