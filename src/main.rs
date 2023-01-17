use mlua::{Error as LuaError, Lua};
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::modules::require;

mod assets;
mod modules;
mod private;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), LuaError> {
    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_target(false)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let lua = Lua::new();

    let globals = lua.globals();

    globals.set("require_ref", globals.get::<_, mlua::Function>("require")?)?;
    globals.set("require", lua.create_async_function(require)?)?;
    globals.set("__INTERNAL_LOADED_MODULES", lua.create_table()?)?;

    std::env::set_current_dir("./src").unwrap();
    if let Err(e) = lua.load(&std::fs::read_to_string("script.lua")?).exec_async().await {
        println!("{:#?}", e);
    }
    Ok(())
}
