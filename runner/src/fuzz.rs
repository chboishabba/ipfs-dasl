use rand::Rng;
use std::fs;
use std::path::PathBuf;

pub fn load_seed_files(dir: &str) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .ok()
        .into_iter()
        .flat_map(|r| r)
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();
    files.sort();
    files
}

pub fn mutate(input: &[u8], format: &str, rng: &mut impl Rng) -> Vec<u8> {
    match format {
        "cid" => mutate_cid(input, rng),
        "car" => mutate_car(input, rng),
        _ => mutate_drisl(input, rng),
    }
}

fn mutate_drisl(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    if input.is_empty() {
        return random_append(input, rng);
    }

    match rng.gen_range(0..5) {
        0 => truncate(input, rng),
        1 => bitflip(input, rng),
        2 => random_append(input, rng),
        3 => duplicate_region(input, rng),
        _ => drop_random_chunk(input, rng),
    }
}

fn mutate_cid(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    if input.is_empty() {
        return random_append(input, rng);
    }
    // CID bytes are fixed-length; prefer bit flips and tiny truncations.
    match rng.gen_range(0..3) {
        0 => bitflip(input, rng),
        1 => truncate(input, rng),
        _ => random_append(input, rng),
    }
}

fn mutate_car(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    if input.is_empty() {
        return random_append(input, rng);
    }
    // CAR headers are CBOR; keep mutations small to preserve structure sometimes.
    match rng.gen_range(0..4) {
        0 => bitflip(input, rng),
        1 => truncate(input, rng),
        2 => drop_random_chunk(input, rng),
        _ => random_append(input, rng),
    }
}

fn truncate(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    let cut = rng.gen_range(0..input.len());
    input[..cut].to_vec()
}

fn bitflip(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    let mut out = input.to_vec();
    let i = rng.gen_range(0..out.len());
    let bit = 1u8 << rng.gen_range(0..8);
    out[i] ^= bit;
    out
}

fn random_append(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    let mut out = input.to_vec();
    let extra = rng.gen_range(1..=16);
    for _ in 0..extra {
        out.push(rng.gen());
    }
    out
}

fn duplicate_region(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    let mut out = input.to_vec();
    if input.len() < 2 {
        return random_append(input, rng);
    }
    let start = rng.gen_range(0..input.len() - 1);
    let end = rng.gen_range(start + 1..input.len());
    let slice = &input[start..end];
    let insert_at = rng.gen_range(0..=out.len());
    out.splice(insert_at..insert_at, slice.iter().cloned());
    out
}

fn drop_random_chunk(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    if input.len() < 2 {
        return truncate(input, rng);
    }
    let start = rng.gen_range(0..input.len() - 1);
    let end = rng.gen_range(start + 1..input.len());
    let mut out = Vec::with_capacity(input.len() - (end - start));
    out.extend_from_slice(&input[..start]);
    out.extend_from_slice(&input[end..]);
    out
}
