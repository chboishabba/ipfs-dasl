use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde::Deserialize;
use wait_timeout::ChildExt;

#[derive(Debug, Deserialize)]
pub struct TargetOutput {
    pub impl_id: String,
    pub accepted: bool,
    pub error_class: String,
    pub canonical_bytes: Option<String>,
    pub time_ms: u128,
}

pub fn run_target(cmd: &str, input: &[u8], timeout_ms: u64, format: &str) -> Result<TargetOutput, String> {
    let mut parts = cmd.split_whitespace();
    let program = parts.next().ok_or_else(|| "empty command".to_string())?;
    let args: Vec<&str> = parts.collect();

    let mut child = Command::new(program)
        .args(args)
        .env("DASL_FORMAT", format)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn failed: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input)
            .map_err(|e| format!("stdin write failed: {e}"))?;
    }

    let start = Instant::now();
    let status = child
        .wait_timeout(Duration::from_millis(timeout_ms))
        .map_err(|e| format!("wait failed: {e}"))?;

    if status.is_none() {
        let _ = child.kill();
        return Err("timeout".to_string());
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("output failed: {e}"))?;

    let elapsed_ms = start.elapsed().as_millis();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let mut parsed: TargetOutput = serde_json::from_str(&stdout)
        .map_err(|e| format!("invalid json output: {e}; stdout={stdout}"))?;

    // Ensure time_ms is set if target didn't.
    if parsed.time_ms == 0 {
        parsed.time_ms = elapsed_ms;
    }

    Ok(parsed)
}
