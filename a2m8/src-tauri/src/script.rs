use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

impl Ord for A2M8Script {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.favorite.cmp(&other.favorite)
    }
}

impl PartialOrd for A2M8Script {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl A2M8Script {
    const STATUS_RUNNING: i8 = 1;
    #[allow(dead_code)]
    const STATUS_STOPPED: i8 = 2;
    #[allow(dead_code)]
    const STATUS_ENDED: i8 = 3;
    #[allow(dead_code)]
    const STATUS_ERROR: i8 = 4;
    pub fn running(&self) -> bool {
        self.status == Self::STATUS_RUNNING
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
