use sha2::{Digest, Sha256};
use rand::Rng;
use std::{env, fs, time::SystemTime};

use differential_runner::{RunRecord, TargetResult};

mod config;
mod exec;
mod fuzz;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "usage: differential-runner <input-file|dir> [case-id] [format] [targets.toml]\n       differential-runner fuzz <seed-dir> [targets.toml]"
        );
        std::process::exit(2);
    }

    let mode = &args[1];
    let targets_path = if mode == "fuzz" {
        args.get(3)
            .cloned()
            .unwrap_or_else(|| "./targets/targets.toml".to_string())
    } else if mode == "run" {
        args.get(5)
            .cloned()
            .unwrap_or_else(|| "./targets/targets.toml".to_string())
    } else {
        args.get(4)
            .cloned()
            .unwrap_or_else(|| "./targets/targets.toml".to_string())
    };

    let cfg_text = fs::read_to_string(&targets_path).expect("failed to read targets config");
    let cfg: config::TargetsConfig = toml::from_str(&cfg_text).expect("invalid targets.toml");

    if mode == "fuzz" {
        let seed_dir = args.get(2).expect("missing seed dir").clone();
        run_fuzz(&seed_dir, &cfg);
        return;
    }

    if mode == "run" {
        let input_path = args.get(2).expect("missing input path").clone();
        let case_id = args.get(3).cloned().unwrap_or_else(|| "manual".to_string());
        let format = args.get(4).cloned().unwrap_or_else(|| "drisl1".to_string());
        dispatch_run(&input_path, &case_id, &format, &cfg);
        return;
    }

    let input_path = mode.to_string();
    let case_id = args.get(2).cloned().unwrap_or_else(|| "manual".to_string());
    let format = args.get(3).cloned().unwrap_or_else(|| "drisl1".to_string());

    dispatch_run(&input_path, &case_id, &format, &cfg);
}

fn dispatch_run(input_path: &str, case_id: &str, format: &str, cfg: &config::TargetsConfig) {
    let input_meta = fs::metadata(input_path).expect("failed to stat input");
    if input_meta.is_dir() {
        run_dir(input_path, &cfg);
    } else {
        let record = run_single(input_path, case_id, format, cfg);
        let json = serde_json::to_string_pretty(&record).expect("failed to serialize record");
        println!("{}", json);
    }
}

fn run_dir(dir: &str, cfg: &config::TargetsConfig) {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .expect("failed to read dir")
        .filter_map(|e| e.ok())
        .collect();
    entries.sort_by_key(|e| e.file_name());

    fs::create_dir_all("reports").ok();
    let mut report_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("reports/report.jsonl")
        .expect("failed to open report.jsonl");

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let case_id = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("case")
            .to_string();
        let fmt = detect_format(&case_id);
        let record = run_single(path.to_str().unwrap(), &case_id, fmt, cfg);
        let jsonl = serde_json::to_string(&record).expect("failed to serialize record");
        use std::io::Write;
        writeln!(report_file, "{}", jsonl).expect("failed to write report");
        println!("{}", jsonl);
        maybe_store_witness(&record, path.to_str().unwrap());
    }
}

fn run_single(path: &str, case_id: &str, format: &str, cfg: &config::TargetsConfig) -> RunRecord {
    let bytes = fs::read(path).expect("failed to read input file");
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let input_hash = hex::encode(hasher.finalize());

    let mut results: Vec<TargetResult> = Vec::new();
    for target in &cfg.target {
        let output = exec::run_target(&target.cmd, &bytes, 200, format);
        match output {
            Ok(out) => {
                results.push(TargetResult {
                    impl_id: out.impl_id,
                    accepted: out.accepted,
                    error_class: out.error_class,
                    canonical_bytes: out.canonical_bytes,
                    time_ms: out.time_ms,
                });
            }
            Err(err) => {
                results.push(TargetResult {
                    impl_id: target.name.clone(),
                    accepted: false,
                    error_class: err,
                    canonical_bytes: None,
                    time_ms: 0,
                });
            }
        }
    }

    let invariant_status = compute_invariant(&results);
    let idempotence_status = compute_idempotence(&results, cfg, 200, format);
    let divergence_notes = compute_divergence_notes(&results, &invariant_status, &idempotence_status);

    let run_id = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| format!("{}", d.as_millis()))
        .unwrap_or_else(|_| "0".to_string());

    RunRecord {
        run_id,
        case_id: case_id.to_string(),
        format: format.to_string(),
        input_hash,
        results,
        invariant_status,
        idempotence_status,
        divergence_notes,
    }
}

fn run_fuzz(seed_dir: &str, cfg: &config::TargetsConfig) {
    const MAX_ITERATIONS: usize = 20_000;
    const MAX_INPUT_SIZE: usize = 256 * 1024;
    const MAX_FAILURES: usize = 50;
    let start = std::time::Instant::now();
    let max_duration = std::time::Duration::from_secs(600);

    let seeds = fuzz::load_seed_files(seed_dir);
    if seeds.is_empty() {
        eprintln!("no seeds found in {}", seed_dir);
        return;
    }

    let mut rng = rand::thread_rng();
    let mut failures = 0usize;

    for i in 0..MAX_ITERATIONS {
        if failures >= MAX_FAILURES || start.elapsed() > max_duration {
            break;
        }
        let seed_path = seeds[rng.gen_range(0..seeds.len())].clone();
        let seed_bytes = fs::read(&seed_path).unwrap_or_default();
        let format = detect_format(seed_path.file_name().and_then(|s| s.to_str()).unwrap_or("seed"));
        let mutated = fuzz::mutate(&seed_bytes, format, &mut rng);
        if mutated.len() > MAX_INPUT_SIZE {
            continue;
        }

        let tmp_path = "/tmp/dasl_fuzz_input.bin";
        fs::write(tmp_path, &mutated).ok();
        let record = run_single(tmp_path, "fuzz", format, cfg);
        let failed = is_failure(&record);
        if failed {
            failures += 1;
            let minimized = minimize_input(&mutated, cfg);
            maybe_store_witness_bytes(&record, &minimized);
        }

        if i % 1000 == 0 && i > 0 {
            eprintln!("fuzz progress: {} iterations, {} failures", i, failures);
        }
    }
}

fn compute_invariant(results: &[TargetResult]) -> String {
    let accepted: Vec<&TargetResult> = results.iter().filter(|r| r.accepted).collect();
    if accepted.len() < 2 {
        return "not_applicable".to_string();
    }
    let first = &accepted[0].canonical_bytes;
    for r in accepted.iter().skip(1) {
        if r.canonical_bytes != *first {
            return "fail".to_string();
        }
    }
    "pass".to_string()
}

fn compute_idempotence(
    results: &[TargetResult],
    cfg: &config::TargetsConfig,
    timeout_ms: u64,
    format: &str,
) -> String {
    let accepted: Vec<&TargetResult> = results.iter().filter(|r| r.accepted).collect();
    if accepted.is_empty() {
        return "not_applicable".to_string();
    }

    for res in accepted {
        let canonical_hex = match &res.canonical_bytes {
            Some(v) => v,
            None => return "fail".to_string(),
        };
        let canonical_bytes = match hex::decode(canonical_hex) {
            Ok(b) => b,
            Err(_) => return "fail".to_string(),
        };
        let target_cmd = cfg
            .target
            .iter()
            .find(|t| t.name == res.impl_id)
            .map(|t| t.cmd.clone())
            .unwrap_or_default();
        if target_cmd.is_empty() {
            return "fail".to_string();
        }
        let rerun = exec::run_target(&target_cmd, &canonical_bytes, timeout_ms, format);
        match rerun {
            Ok(out) => {
                if !out.accepted || out.canonical_bytes.as_deref() != Some(canonical_hex) {
                    return "fail".to_string();
                }
            }
            Err(_) => return "fail".to_string(),
        }
    }
    "pass".to_string()
}

fn detect_format(name: &str) -> &str {
    if name.starts_with("cid_") {
        "cid"
    } else if name.starts_with("car_") {
        "car"
    } else {
        "drisl1"
    }
}

fn compute_divergence_notes(
    results: &[TargetResult],
    invariant_status: &str,
    idempotence_status: &str,
) -> Option<String> {
    let accepts = results.iter().filter(|r| r.accepted).count();
    let rejects = results.len().saturating_sub(accepts);
    if accepts > 0 && rejects > 0 {
        return Some("validity_disagreement".to_string());
    }
    if invariant_status == "fail" {
        return Some("canonical_disagreement".to_string());
    }
    if idempotence_status == "fail" {
        return Some("idempotence_failure".to_string());
    }
    None
}

fn is_failure(record: &RunRecord) -> bool {
    record.invariant_status == "fail"
        || record.idempotence_status == "fail"
        || record.divergence_notes.is_some()
}

fn maybe_store_witness(record: &RunRecord, input_path: &str) {
    if !is_failure(record) {
        return;
    }

    let dir = format!("corpus/regressions/{}", record.input_hash);
    if fs::create_dir_all(&dir).is_err() {
        return;
    }

    let _ = fs::copy(input_path, format!("{}/input.bin", dir));
    write_results(record, &dir);
}

fn maybe_store_witness_bytes(record: &RunRecord, input: &[u8]) {
    if !is_failure(record) {
        return;
    }

    let dir = format!("corpus/regressions/{}", record.input_hash);
    if fs::create_dir_all(&dir).is_err() {
        return;
    }

    let _ = fs::write(format!("{}/input.bin", dir), input);
    write_results(record, &dir);
}

fn write_results(record: &RunRecord, dir: &str) {
    if let Ok(json) = serde_json::to_string_pretty(record) {
        let _ = fs::write(format!("{}/results.json", dir), json);
    }

    let canon_dir = format!("{}/canon", dir);
    let _ = fs::create_dir_all(&canon_dir);
    for res in &record.results {
        if res.accepted {
            if let Some(hex_bytes) = &res.canonical_bytes {
                if let Ok(bytes) = hex::decode(hex_bytes) {
                    let _ = fs::write(format!("{}/{}.bin", canon_dir, res.impl_id), bytes);
                }
            }
        }
    }
}

fn minimize_input(input: &[u8], cfg: &config::TargetsConfig) -> Vec<u8> {
    if input.len() <= 1 {
        return input.to_vec();
    }
    let mut current = input.to_vec();
    let mut chunk_size = current.len() / 2;

    while chunk_size > 0 {
        let mut reduced = false;
        let mut i = 0usize;
        while i < current.len() {
            let end = std::cmp::min(i + chunk_size, current.len());
            let mut candidate = Vec::with_capacity(current.len() - (end - i));
            candidate.extend_from_slice(&current[..i]);
            candidate.extend_from_slice(&current[end..]);
            if candidate.is_empty() {
                i += chunk_size;
                continue;
            }
            if failure_persists(&candidate, cfg) {
                current = candidate;
                reduced = true;
                break;
            }
            i += chunk_size;
        }
        if !reduced {
            chunk_size /= 2;
        }
    }
    current
}

fn failure_persists(input: &[u8], cfg: &config::TargetsConfig) -> bool {
    let tmp_path = "/tmp/dasl_fuzz_min.bin";
    let _ = fs::write(tmp_path, input);
    let record = run_single(tmp_path, "fuzz_min", "drisl1", cfg);
    is_failure(&record)
}
