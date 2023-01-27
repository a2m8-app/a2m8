use cli_clipboard::{ClipboardContext, ClipboardProvider};
use mlua::UserData;

use crate::create_body;
use crate::prelude::*;

pub fn init(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    create_body!(lua,
        "clipboard" => Clipboard {}
    )
}

pub struct Clipboard {}

impl Clipboard {
    fn set_contents(value: String) -> Result<(), mlua::Error> {
        #[cfg(not(target_os = "linux"))]
        {
            let mut ctx = ClipboardContext::new().map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;
            ctx.set_contents(value)
                .map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;
        }
        #[cfg(target_os = "linux")]
        {
            use clipboard_ext::{prelude::*, x11_fork::ClipboardContext};
            fn set_clipboard(value: String) -> Result<(), Box<dyn std::error::Error>> {
                let mut ctx = ClipboardContext::new()?;
                ctx.set_contents(value)?;
                Ok(())
            }
            if set_clipboard(value.clone()).is_err() {
                let mut ctx = ClipboardContext::new().map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;
                ctx.set_contents(value)
                    .map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;
            }
        }
        Ok(())
    }
    fn get_contents() -> Result<String, mlua::Error> {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.get_contents().map_err(|x| mlua::Error::RuntimeError(x.to_string()))
    }
    fn clear_contents() -> Result<(), mlua::Error> {
        let mut ctx = ClipboardContext::new().map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;
        ctx.clear().map_err(|x| mlua::Error::RuntimeError(x.to_string()))
    }
}

impl UserData for Clipboard {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, _| Self::get_contents());
        fields.add_field_method_set("value", |_, _, value: String| Self::set_contents(value));
    }
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("set", |_, value: String| Self::set_contents(value));
        methods.add_function("get", |_, ()| Self::get_contents());
        methods.add_function("clear", |_, ()| Self::clear_contents())
    }
}
