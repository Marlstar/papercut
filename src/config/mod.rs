use std::path::PathBuf;
use crate::Backend;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    backend: Backend,
    #[serde(skip)]
    installed_backends: Vec<Backend>,
    directory: PathBuf,
}
impl Config {
    pub async fn load() -> Self {
        ron::de::from_reader()
    }
}
