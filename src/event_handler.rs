use std::collections::HashMap;

use mlua::{FromLua, Function, UserData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
enum Events {
    Click,
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

pub struct EventHandler<'a> {
    event_handlers: HashMap<Events, Box<Function<'a>>>,
}

impl UserData for EventHandler<'_> {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("on", |_, this, (event, function): (Events, Function)| {
            this.event_handlers.insert(event, Box::new(function));
            Ok(())
        });
    }

    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}
}
