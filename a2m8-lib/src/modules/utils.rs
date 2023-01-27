use std::time::Instant;

use mlua::{Error as LuaError, Lua, UserData};
use tokio::time;

use crate::create_body;

use super::command;

pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body! (lua,
        "sleep" => lua.create_async_function(sleep)?,
        "performance" => Performance::new(),
        "open_app" => lua.create_async_function(open_app)?,
        "find_app" => lua.create_function(find_app)?
    )
}

fn find_app(_: &Lua, appname: String) -> mlua::Result<String> {
    let app = appfinder::get_app_path(&appname);
    if let Some(app) = app {
        Ok(appfinder::remove_arguments(&app))
    } else {
        Err(LuaError::RuntimeError(format!("Could not find app {appname}")))
    }
}

async fn open_app(lua: &Lua, (app, cwd): (String, Option<String>)) -> mlua::Result<()> {
    let app = appfinder::get_app_path(&app);
    if let Some(app) = app {
        command::run_command(lua, (app, cwd)).await?;
    }
    Ok(())
}

pub async fn sleep(_: &Lua, time: f64) -> Result<(), LuaError> {
    time::sleep(time::Duration::from_millis((time * 1000.0) as u64)).await;
    Ok(())
}

#[derive(Clone)]
pub struct Performance(Instant);

impl Performance {
    pub fn new() -> Self {
        Self(Instant::now())
    }

    pub fn elapsed(&self) -> f64 {
        self.0.elapsed().as_secs_f64()
    }
    pub fn since(&self, perf: Performance) -> f64 {
        self.0.saturating_duration_since(perf.0).as_secs_f64()
    }
}

impl Default for Performance {
    fn default() -> Self {
        Self::new()
    }
}

impl UserData for Performance {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("new", |_, ()| Ok(Performance::new()));
        methods.add_function("now", |_, ()| Ok(Performance::new()));
        methods.add_method("elapsed", |_, this, ()| Ok(this.elapsed()));
        methods.add_method("since", |_, this, perf: Performance| Ok(this.since(perf)));
    }
}
