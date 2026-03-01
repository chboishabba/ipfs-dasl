use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Outcome {
    Accept { canonical_bytes: String },
    Reject { error_class: String, stderr: String },
    Crash { detail: String },
    Timeout,
}

#[derive(Debug, Serialize)]
pub struct TargetResult {
    pub impl_id: String,
    pub accepted: bool,
    pub error_class: String,
    pub canonical_bytes: Option<String>,
    pub time_ms: u128,
}

#[derive(Debug, Serialize)]
pub struct RunRecord {
    pub run_id: String,
    pub case_id: String,
    pub format: String,
    pub input_hash: String,
    pub results: Vec<TargetResult>,
    pub invariant_status: String,
    pub idempotence_status: String,
    pub divergence_notes: Option<String>,
}
