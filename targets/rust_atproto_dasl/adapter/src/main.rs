use std::io::{self, Read};
use std::time::Instant;

fn main() {
    let mut input = Vec::new();
    let _ = io::stdin().read_to_end(&mut input);

    let format = std::env::var("DASL_FORMAT").unwrap_or_else(|_| "drisl1".to_string());
    let start = Instant::now();

    let result = match format.as_str() {
        "cid" => {
            match atproto_dasl::cid::Cid::from_bytes(&input) {
                Ok(cid) => Ok(cid.to_bytes()),
                Err(e) => Err(e.to_string()),
            }
        }
        "car" => {
            match atproto_dasl::car::CarHeader::from_bytes(&input) {
                Ok((header, _)) => header.to_bytes().map_err(|e| e.to_string()),
                Err(e) => Err(e.to_string()),
            }
        }
        _ => {
            match atproto_dasl::from_slice::<atproto_dasl::Ipld>(&input) {
                Ok(value) => atproto_dasl::to_vec(&value).map_err(|e| e.to_string()),
                Err(e) => Err(e.to_string()),
            }
        }
    };

    let elapsed = start.elapsed().as_millis();

    match result {
        Ok(canonical) => {
            let hex_bytes = hex::encode(canonical);
            println!(
                "{}",
                serde_json::json!({
                    "impl_id": "atproto-dasl",
                    "accepted": true,
                    "error_class": "none",
                    "canonical_bytes": hex_bytes,
                    "time_ms": elapsed
                })
            );
        }
        Err(err) => {
            println!(
                "{}",
                serde_json::json!({
                    "impl_id": "atproto-dasl",
                    "accepted": false,
                    "error_class": "decode_error",
                    "canonical_bytes": null,
                    "time_ms": elapsed,
                    "stderr": err
                })
            );
        }
    }
}
