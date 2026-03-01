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
        return boundary_bytes();
    }

    let roll: u8 = rng.gen_range(0..100);
    if roll < 40 {
        boundary_replace(input, rng)
    } else if roll < 70 {
        truncate(input, rng)
    } else if roll < 90 {
        random_append(input, rng)
    } else {
        if rng.gen_bool(0.5) {
            bitflip(input, rng)
        } else {
            duplicate_region(input, rng)
        }
    }
}

fn mutate_cid(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    let roll: u8 = rng.gen_range(0..100);
    if roll < 60 {
        cid_boundary(input, rng)
    } else if roll < 80 {
        truncate_or_pad(input, rng)
    } else {
        bitflip(input, rng)
    }
}

fn mutate_car(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    let roll: u8 = rng.gen_range(0..100);
    if roll < 50 {
        car_boundary(input, rng)
    } else if roll < 70 {
        truncate(input, rng)
    } else if roll < 90 {
        bitflip(input, rng)
    } else {
        random_append(input, rng)
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

fn boundary_replace(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    let mut out = input.to_vec();
    let boundary = [0x17u8, 0x18, 0x19, 0x1a, 0x1b, 0x58, 0x59, 0x5a, 0x9f, 0xbf, 0xff];
    let i = rng.gen_range(0..out.len());
    out[i] = boundary[rng.gen_range(0..boundary.len())];
    out
}

fn boundary_bytes() -> Vec<u8> {
    vec![0x18, 0x00, 0x9f, 0xff]
}

fn truncate_or_pad(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    let mut out = input.to_vec();
    if out.len() >= 36 && rng.gen_bool(0.5) {
        out.truncate(35);
        return out;
    }
    if out.len() >= 36 {
        out.push(rng.gen());
        return out;
    }
    random_append(input, rng)
}

fn cid_boundary(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    let mut out = if input.len() >= 36 { input[..36].to_vec() } else { input.to_vec() };
    if out.len() < 4 {
        return truncate_or_pad(&out, rng);
    }
    match rng.gen_range(0..5) {
        0 => out[0] = if out[0] == 0x01 { 0x00 } else { 0x01 },
        1 => out[1] = if out[1] == 0x55 { 0x70 } else { 0x55 },
        2 => out[2] = if out[2] == 0x12 { 0x13 } else { 0x12 },
        3 => out[3] = if out[3] == 0x20 { 0x1f } else { 0x20 },
        _ => return truncate_or_pad(&out, rng),
    }
    out
}

fn car_boundary(input: &[u8], rng: &mut impl Rng) -> Vec<u8> {
    let mut out = input.to_vec();
    if out.is_empty() {
        return boundary_bytes();
    }

    // try to flip the version byte if "version" key is present
    if let Some(pos) = find_subslice(&out, b"version") {
        let val_idx = pos + b"version".len();
        if val_idx < out.len() {
            out[val_idx] = if out[val_idx] == 0x01 { 0x02 } else { 0x01 };
            return out;
        }
    }

    // fallback: replace a random byte with boundary values
    boundary_replace(&out, rng)
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}
