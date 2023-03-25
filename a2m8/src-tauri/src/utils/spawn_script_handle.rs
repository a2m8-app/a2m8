use tauri::async_runtime::TokioJoinHandle as JoinHandle;
use tokio::{
    process::Child,
    select,
    sync::{mpsc, oneshot},
};
use uuid::Uuid;

use crate::{script::A2M8Script, Result, ScriptEnd};

pub fn spawn_script_handle(
    tx: mpsc::Sender<ScriptEnd>,
    stop_receiver: oneshot::Receiver<u8>,
    mut child: Child,
    id: Uuid,
) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        let mut status = A2M8Script::STATUS_STOPPED;
        let mut error = None;
        select! {
            _ = stop_receiver => {
                //we dont care about the result
                child.kill().await.ok();
            }
            res = child.wait() => {
                match res {
                    Err(err) => {
                       status = A2M8Script::STATUS_ERROR;
                       error = Some(err.to_string());
                    }
                    Ok(s) => {
                        match s.code() {
                            Some(0) => {
                                status = A2M8Script::STATUS_ENDED;
                            }
                            _ => {
                                status = A2M8Script::STATUS_ERROR;
                            }
                        }
                    }
                }

            }
        }
        //we dont care about the result
        tx.send(ScriptEnd { id, status, error }).await.ok();
        Ok(())
    })
}
