use std::time::SystemTime;

use mlua::Lua;
use rdev::{simulate, Button, Event, EventType};

use crate::create_body;

use super::event_handler::{parse_key, EventEvent};

#[doc(hidden)]
pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body! (lua,
        "create_mouse_move" => lua.create_function(create_mouse_move)?,
        "create_wheel" => lua.create_function(create_wheel)?,
        "create_key_press" => lua.create_function(create_key_press)?,
        "create_key_release" => lua.create_function(create_key_release)?,
        "create_button_press" => lua.create_function(create_button_press)?,
        "create_button_release" => lua.create_function(create_button_release)?,
        "simulate" => lua.create_function(simulate_event)?
    )
}

fn ev(event_type: EventType) -> mlua::Result<EventEvent> {
    Ok(EventEvent(Event {
        time: SystemTime::now(),
        name: None,
        event_type,
    }))
}

pub fn create_mouse_move(_: &Lua, (x, y): (f64, f64)) -> mlua::Result<EventEvent> {
    ev(EventType::MouseMove { x, y })
}

pub fn create_wheel(_: &Lua, (x, y): (i64, i64)) -> mlua::Result<EventEvent> {
    ev(EventType::Wheel { delta_x: x, delta_y: y })
}

pub fn create_key_press(_: &Lua, key: String) -> mlua::Result<EventEvent> {
    ev(EventType::KeyPress(parse_key(key)?))
}

pub fn create_key_release(_: &Lua, key: String) -> mlua::Result<EventEvent> {
    ev(EventType::KeyRelease(parse_key(key)?))
}

pub fn create_button_press(_: &Lua, button: String) -> mlua::Result<EventEvent> {
    ev(EventType::ButtonPress(match button.as_str() {
        "left" => Button::Left,
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => Button::Unknown(button.parse().unwrap()),
    }))
}

pub fn create_button_release(_: &Lua, button: String) -> mlua::Result<EventEvent> {
    ev(EventType::ButtonRelease(match button.as_str() {
        "left" => Button::Left,
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => Button::Unknown(button.parse().unwrap()),
    }))
}

pub fn simulate_event(_: &Lua, event: EventEvent) -> mlua::Result<()> {
    simulate(&event.0.event_type).map_err(|e| mlua::Error::RuntimeError(e.to_string()))
}
