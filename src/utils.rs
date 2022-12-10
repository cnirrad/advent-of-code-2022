use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

pub fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path).with_context(|| format!("Could not find {}", path))?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    Ok(lines)
}

pub fn read_file_as_string_vec(path: &str) -> Result<Vec<String>> {
    let lines = read_file(path)?;

    let v: Vec<String> = lines.split('\n').map(|s| s.to_owned()).collect();

    Ok(v)
}
