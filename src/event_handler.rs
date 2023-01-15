use mlua::{FromLua, UserData, UserDataMethods};
use rdev::{Button, Event, EventType};
use serde::{Deserialize, Serialize};

use crate::EVENT_LISTENER;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Events {
    Click,
    KeyPress,
    KeyRelease,
    MouseMove,
    Wheel,
}

impl Events {
    pub fn to_string(&self) -> &'static str {
        match self {
            Events::Click => "click",
            Events::KeyPress => "key_press",
            Events::KeyRelease => "key_release",
            Events::MouseMove => "mouse_move",
            Events::Wheel => "wheel",
        }
    }
}

impl Default for Events {
    fn default() -> Self {
        Events::Click
    }
}
impl<'lua> FromLua<'lua> for Events {
    fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let ty = lua_value.type_name();
        let string = lua
            .coerce_string(lua_value)?
            .ok_or_else(|| mlua::Error::FromLuaConversionError {
                from: ty,
                to: "String",
                message: Some("expected string or number".to_string()),
            })?
            .to_str()?
            .to_owned();

        match string.as_str() {
            "click" => Ok(Events::Click),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "string",
                to: "Events",
                message: Some("Invalid event".to_string()),
            }),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EventEvent(Event);
impl EventEvent {
    pub fn name(&self) -> &'static str {
        match self.0.event_type {
            EventType::KeyPress { .. } => "key_press",
            EventType::KeyRelease { .. } => "key_release",
            EventType::ButtonPress(_) => "button_press",
            EventType::ButtonRelease(_) => "button_release",
            EventType::MouseMove { .. } => "mouse_move",
            EventType::Wheel { .. } => "wheel",
        }
    }
}

impl UserData for EventEvent {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name()));
        fields.add_field_method_get("key", |_, this| {
            Ok(match this.0.event_type {
                EventType::KeyPress(key) | EventType::KeyRelease(key) => {
                    serde_json::to_string(&key).unwrap().replace('"', "")
                }
                _ => "".to_string(),
            })
        });
        fields.add_field_method_get("button", |_, this| {
            fn button_to_name(button: Button) -> String {
                match button {
                    Button::Left => "left".to_owned(),
                    Button::Right => "right".to_owned(),
                    Button::Middle => "middle".to_owned(),
                    Button::Unknown(n) => n.to_string(),
                }
            }
            Ok(match this.0.event_type {
                EventType::ButtonPress(button) => button_to_name(button),
                EventType::ButtonRelease(button) => button_to_name(button),
                _ => "".to_string(),
            })
        });
        fields.add_field_method_get("x", |_, this| {
            Ok(match this.0.event_type {
                EventType::MouseMove { x, .. } => x,
                _ => 0.0,
            })
        });
        fields.add_field_method_get("y", |_, this| {
            Ok(match this.0.event_type {
                EventType::MouseMove { y, .. } => y,
                _ => 0.0,
            })
        });
        fields.add_field_method_get("delta_x", |_, this| {
            Ok(match this.0.event_type {
                EventType::Wheel { delta_x, .. } => delta_x,
                _ => 0,
            })
        });
        fields.add_field_method_get("delta_y", |_, this| {
            Ok(match this.0.event_type {
                EventType::Wheel { delta_y, .. } => delta_y,
                _ => 0,
            })
        });
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EventHandler {}

impl UserData for EventHandler {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // methods.add_async_method("read", |lua, this, ()| async move {
        //     let event = EVENT_LISTENER.lock().await.try_recv();
        //     match event {
        //         Ok(event) => Ok(lua.create_ser_userdata(EventEvent(event))?),
        //         Err(_) => Err(mlua::Error::RuntimeError(
        //             "Could no receive event".to_string(),
        //         )),
        //     }
        // });
        methods.add_async_function("read", |lua, ()| async move {
            let event = EVENT_LISTENER.lock().await.recv().await;
            match event {
                Some(event) => Ok(lua.create_ser_userdata(EventEvent(event))?),
                None => Err(mlua::Error::RuntimeError("Could no receive event".to_string())),
            }
        });
        // methods.add_async_function(
        //     "on",
        //     |lua, (event, handler): (Events, Function)| async move {
        //         let mut event_listeners = EVENT_LISTENER.lock().await;
        //         let fun = Arc::new(handler);
        //         tokio::spawn(async move {
        //             while let Some(e) = event_listeners.recv().await {
        //                 let ev = EventEvent(e);
        //                 if event.to_string() == ev.name() {
        //                     let _ = fun.call::<_, ()>(EventEvent(e));
        //                 }
        //             }
        //         });
        //         Ok(())
        //     },
        // )
    }
}
