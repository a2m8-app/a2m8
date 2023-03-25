use crate::prelude::*;

#[tauri::command]
pub async fn create_script(config: tauri::State<'_, A2>, script: A2M8Script) -> Result<()> {
    let mut config = config.lock().await;
    config.create_script(script).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_script(config: tauri::State<'_, A2>, script: A2M8Script) -> Result<()> {
    let mut config = config.lock().await;
    config.update_script(script).await?;
    Ok(())
}

#[tauri::command]
pub async fn delete_script(config: tauri::State<'_, A2>, id: Uuid) -> Result<()> {
    let mut config = config.lock().await;
    config.delete_script(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_scripts(config: tauri::State<'_, A2>) -> Result<Vec<A2M8Script>> {
    let config = config.lock().await;
    Ok(config.scripts.clone())
}

#[tauri::command]
pub async fn start_script(config: tauri::State<'_, A2>, id: Uuid) -> Result<()> {
    let mut config = config.lock().await;
    let script = config
        .scripts
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| anyhow::anyhow!("Script not found"))?
        .clone();

    config.run_script(script).await?;

    Ok(())
}

#[tauri::command]
pub async fn stop_script(config: tauri::State<'_, A2>, id: Uuid) -> Result<()> {
    let mut config = config.lock().await;
    config.stop_script(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_script(config: tauri::State<'_, A2>, id: Uuid) -> Result<A2M8Script> {
    let config = config.lock().await;
    let script = config
        .scripts
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| anyhow::anyhow!("Script not found"))?;
    Ok(script.clone())
}
