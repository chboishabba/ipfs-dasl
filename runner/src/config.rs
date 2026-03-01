use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TargetsConfig {
    pub target: Vec<TargetEntry>,
}

#[derive(Debug, Deserialize)]
pub struct TargetEntry {
    pub name: String,
    pub cmd: String,
}
