use std::collections::HashMap;

use deno_task_shell::{ShellPipeReader, ShellPipeWriter};
use mlua::{Lua, UserData};
use std::io::Read;

use crate::create_body;

#[doc(hidden)]
pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body!(lua,
        "run_command" => lua.create_async_function(run_command)?,
        "run_command_piped" => lua.create_async_function(run_command_piped)?
    )
}

pub async fn run_command(_: &Lua, (command, cwd): (String, Option<String>)) -> mlua::Result<CommandResult> {
    let list = deno_task_shell::parser::parse(&command).map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;

    let default_cwd = std::env::current_dir()?;
    let cwd = cwd.map(|x| x.into()).unwrap_or(default_cwd);
    let env_from_proc = std::env::vars().collect::<HashMap<String, String>>();
    let (mut stdout, stdout_writer) = os_pipe::pipe()?;
    let (mut stderr, stderr_writer) = os_pipe::pipe()?;
    let stdin = ShellPipeReader::stdin();

    let exit_code = deno_task_shell::execute_with_pipes(
        list,
        env_from_proc,
        &cwd,
        stdin,
        ShellPipeWriter::OsPipe(stdout_writer),
        ShellPipeWriter::OsPipe(stderr_writer),
    )
    .await;
    let mut stdout_buffer = Vec::new();
    let mut stderr_buffer = Vec::new();
    stdout.read_to_end(&mut stdout_buffer)?;
    stderr.read_to_end(&mut stderr_buffer)?;
    let stdout = String::from_utf8(stdout_buffer)
        .map_err(|x| mlua::Error::RuntimeError(format!("Failed to convert stdout to utf8: {x}")))?;
    let stderr = String::from_utf8(stderr_buffer)
        .map_err(|x| mlua::Error::RuntimeError(format!("Failed to convert stderr to utf8: {x}")))?;
    Ok(CommandResult {
        stdout,
        stderr,
        exit_code,
    })
}

pub async fn run_command_piped(_: &Lua, (command, cwd): (String, Option<String>)) -> mlua::Result<i32> {
    let list = deno_task_shell::parser::parse(&command).map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;

    let default_cwd = std::env::current_dir()?;
    let cwd = cwd.map(|x| x.into()).unwrap_or(default_cwd);
    let env_from_proc = std::env::vars().collect::<HashMap<String, String>>();

    let exit_code = deno_task_shell::execute(list, env_from_proc, &cwd).await;
    Ok(exit_code)
}

pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl UserData for CommandResult {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("stdout", |_, this: &CommandResult| Ok(this.stdout.clone()));
        fields.add_field_method_get("stderr", |_, this: &CommandResult| Ok(this.stderr.clone()));
        fields.add_field_method_get("exit_code", |_, this: &CommandResult| Ok(this.exit_code));
    }
}
