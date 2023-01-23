use std::path::PathBuf;

use tokio::{fs, sync::mpsc};

use crate::{prelude::*, ScriptEnd};

pub type A2 = Mutex<A2M8Config>;

#[derive(Debug)]
pub struct A2M8Config {
    pub scripts: Vec<A2M8Script>,
    pub data_dir: PathBuf,
    pub stop_sender: mpsc::Sender<ScriptEnd>,
    pub script_handles: Vec<A2M8ScriptRunningHandle>,
}

impl A2M8Config {
    const SCRIPT_FILE: &'static str = "scripts";
    const SCRIPT_BACKUP_FILE: &'static str = "scripts.backup";
    async fn to_vec<T: Serialize>(data: &T) -> Result<Vec<u8>>
    where
        T: Serialize + ?Sized,
    {
        Ok(rmp_serde::to_vec(data)?)
    }
    pub fn from_slice<'a, T>(input: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        Ok(rmp_serde::from_slice(input)?)
    }
    pub async fn load_scripts(&mut self) -> Result<()> {
        let path = self.data_dir.join(Self::SCRIPT_FILE);
        if fs::metadata(&path).await.is_err() {
            fs::write(&path, Self::to_vec(&self.scripts).await?).await?;
        }
        let script = fs::read(path).await?;
        self.scripts = Self::from_slice(&script)?;
        Ok(())
    }
    pub async fn save_scripts(&self) -> Result<()> {
        let script = Self::to_vec(&self.scripts).await?;
        fs::rename(
            self.data_dir.join(Self::SCRIPT_FILE),
            self.data_dir.join(Self::SCRIPT_BACKUP_FILE),
        )
        .await?;
        fs::write(self.data_dir.join(Self::SCRIPT_FILE), script).await?;
        Ok(())
    }
    pub async fn create_script(&mut self, script: A2M8Script) -> Result<()> {
        self.scripts.push(script);
        self.save_scripts().await?;
        Ok(())
    }
    pub async fn update_script(&mut self, script: A2M8Script) -> Result<()> {
        let index = self
            .scripts
            .iter()
            .position(|s| s.id == script.id)
            .ok_or_else(|| anyhow::anyhow!("Script not found"))?;
        self.scripts[index] = script;
        self.save_scripts().await?;
        Ok(())
    }
    pub async fn delete_script(&mut self, id: Uuid) -> Result<()> {
        let index = self
            .scripts
            .iter()
            .position(|s| s.id == id)
            .ok_or_else(|| anyhow::anyhow!("Script not found"))?;
        self.scripts.remove(index);
        self.save_scripts().await?;
        Ok(())
    }
}
