use mlua::Lua;

use crate::create_body;

#[doc(hidden)]
pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body! (lua,
        "trace" => lua.create_function(trace)?,
        "info" => lua.create_function(info)?,
        "debug" => lua.create_function(debug)?,
        "warn" => lua.create_function(warn)?,
        "error" => lua.create_function(error)?
    )
}

macro_rules! create_log_macros {
    ($($name:ident),*) => {
        $(
            pub fn $name(_: &Lua, message: String) -> mlua::Result<()> {
                tracing::$name!("{}", message);
                Ok(())
            }
        )*
    };
}

create_log_macros!(trace, info, debug, warn, error);
