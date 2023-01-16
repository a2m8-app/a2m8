use std::collections::HashMap;

use mlua::Lua;

pub async fn run_command(_: &Lua, (command, cwd): (String, Option<String>)) -> mlua::Result<i32> {
    let list = deno_task_shell::parser::parse(&command).map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;

    let default_cwd = std::env::current_dir()?;
    let cwd = cwd.map(|x| x.into()).unwrap_or(default_cwd);
    let env_from_proc = std::env::vars().collect::<HashMap<String, String>>(); 
    let exit_code = deno_task_shell::execute(list, env_from_proc, &cwd).await;
    Ok(exit_code)
}
