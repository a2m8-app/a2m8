use mlua::Lua;
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;

use a2m8_lib::require;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
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

    let file = std::env::args().last().expect("REQUIRED ARGS");

    lua.load(&std::fs::read_to_string(file)?)
        .set_name("main")?
        .exec_async()
        .await?;
    Ok(())
}
