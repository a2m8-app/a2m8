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
    pub status: ScriptStatus,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ScriptStatus {
    Running = 1,
    Stopped = 2,
    Ended = 3,
    Error = 4,
}
