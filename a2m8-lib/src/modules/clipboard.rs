use cli_clipboard::{ClipboardContext, ClipboardProvider};

use crate::create_body;

pub fn init(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    create_body!(lua,
        "set"=> lua.create_function(set)?,
        "get"=> lua.create_function(get)?,
        "clear" => lua.create_function(clear)?
    )
}

fn set(_: &mlua::Lua, value: String) -> mlua::Result<()> {
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

fn get(_: &mlua::Lua, args: ()) -> mlua::Result<String> {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.get_contents().map_err(|x| mlua::Error::RuntimeError(x.to_string()))
}

fn clear(_: &mlua::Lua, args: ()) -> mlua::Result<()> {
    let mut ctx = ClipboardContext::new().map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;
    ctx.clear().map_err(|x| mlua::Error::RuntimeError(x.to_string()))
}
