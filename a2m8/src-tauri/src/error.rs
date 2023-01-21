use std::result::Result as StdResult;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeRmp(#[from] rmp_serde::decode::Error),
    #[error(transparent)]
    SerdeRmpEncode(#[from] rmp_serde::encode::Error),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
