use std::path::PathBuf;

use tokio::{
    process::{Child, Command},
    sync::oneshot::{self},
};

use crate::prelude::*;

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

impl A2M8Script {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let content = std::fs::read_to_string(path)?;
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description: "".to_string(),
            startup: false,
            favorite: false,
            content,
            error: None,
            status: 0,
        })
    }
}

#[derive(Debug)]
pub struct A2M8ScriptRunningHandle {
    pub id: Uuid,
    pub stop_sender: oneshot::Sender<u8>,
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
    pub const STATUS_STOPPED: i8 = 2;
    pub const STATUS_ENDED: i8 = 3;
    pub const STATUS_ERROR: i8 = 4;
    pub fn running(&self) -> bool {
        self.status == Self::STATUS_RUNNING
    }

    pub async fn start(&mut self) -> Result<Child> {
        let child = Command::new(
            //run this executable again
            std::env::current_exe()?,
        )
        .arg("start")
        .arg(self.id.clone().to_string())
        .spawn()?;

        self.status = Self::STATUS_RUNNING;

        Ok(child)
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
