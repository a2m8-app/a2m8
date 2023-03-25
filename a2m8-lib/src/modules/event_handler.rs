use mlua::{FromLua, Function, Lua, UserData};
use rdev::{Button, Event, EventType, Key};
use serde::{Deserialize, Serialize};

use crate::{
    create_body,
    private::event_listener::{EVENT_GRABBER, EVENT_LISTENER},
};

#[doc(hidden)]
pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body!(lua,
        "read" => lua.create_async_function(read)?,
        "grab" => lua.create_async_function(grab)?
    )
}

async fn read(_: &Lua, _: ()) -> mlua::Result<EventEvent> {
    let event = EVENT_LISTENER.lock().await.recv().await;
    match event {
        Some(event) => Ok(EventEvent(event)),
        None => Err(mlua::Error::RuntimeError("Could no receive event".to_string())),
    }
}

async fn grab<'lua>(_: &'lua Lua, fun: Function<'_>) -> mlua::Result<()> {
    let (event, responder) = match EVENT_GRABBER.lock().await.recv().await {
        Some(event) => event,
        None => return Err(mlua::Error::RuntimeError("Could no receive event".to_string())),
    };

    let result = fun.call_async::<_, Option<EventEvent>>(EventEvent(event)).await?;
    responder
        .send(result.map(|x| x.0))
        .map_err(|_| mlua::Error::RuntimeError("Could no send event".to_string()))?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum Events {
    #[default]
    Click,
    KeyPress,
    KeyRelease,
    MouseMove,
    Wheel,
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
pub struct EventEvent(pub(crate) Event);
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

pub fn parse_key(key: String) -> mlua::Result<Key> {
    serde_json::from_str(&format!("\"{key}\"")).map_err(|e| mlua::Error::FromLuaConversionError {
        from: "string",
        to: "Key",
        message: Some(format!("Invalid key: {e}")),
    })
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
