use std::thread;

use easyhotkeys::require;
use mlua::Lua;
use tokio::{
    select,
    sync::oneshot::{self, Receiver, Sender},
};

use crate::{error::Error, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct A2M8Script {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub startup: bool,
    pub favorite: bool,
    pub content: String,
    pub error: Option<String>,
    pub status: i8,
}

#[derive(Debug)]
pub struct A2M8ScriptRunningHandle {
    pub id: Uuid,
    pub handle: thread::JoinHandle<Result<()>>,
    pub sender: Sender<Result<()>>,
}

// impl Ord for A2M8Script {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.favorite.cmp(&other.favorite)
//     }
// }

// impl PartialOrd for A2M8Script {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

mod script_status {}

impl A2M8Script {
    pub const STATUS_RUNNING: i8 = 1;
    #[allow(dead_code)]
    pub const STATUS_STOPPED: i8 = 2;
    #[allow(dead_code)]
    pub const STATUS_ENDED: i8 = 3;
    #[allow(dead_code)]
    pub const STATUS_ERROR: i8 = 4;
    pub fn running(&self) -> bool {
        self.status == Self::STATUS_RUNNING
    }

    pub async fn start(&mut self) -> Result<(Receiver<Result<()>>, A2M8ScriptRunningHandle)> {
        let content = self.content.clone();
        let name = self.name.clone();
        let (sender, receiver) = oneshot::channel();
        let (finish_sender, finish_receiver) = oneshot::channel();
        let handle = thread::Builder::new().name(name.clone()).spawn(move || -> Result<()> {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?
                .block_on(async {
                    let inner = || async move {
                        let lua = Lua::new();

                        let globals = lua.globals();
                        globals.set("require_ref", globals.get::<_, mlua::Function>("require")?)?;
                        globals.set("require", lua.create_async_function(require)?)?;
                        globals.set("__INTERNAL_LOADED_MODULES", lua.create_table()?)?;
                        select! {
                            _ = receiver => {
                                finish_sender.send(Ok(())).unwrap();
                                println!("Script  stopped",);
                            },
                            res = lua.load(&content).set_name(name)?.exec_async() => {
                                finish_sender.send(res.map_err(Error::from)).unwrap();
                            }
                        }
                        Ok::<_, Error>(())
                    };
                    inner().await?;
                    Ok::<_, Error>(())
                })?;
            Ok(())
        })?;
        self.status = Self::STATUS_RUNNING;

        Ok((
            finish_receiver,
            A2M8ScriptRunningHandle {
                id: self.id,
                handle,
                sender,
            },
        ))
    }
}
// #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
// #[repr(u8)]
// pub enum ScriptStatus {
//     Running = 1,
//     Stopped = 2,
//     Ended = 3,
//     Error = 4,
// }
