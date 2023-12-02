use std::{error::Error, path::Path};

pub fn load_vec<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    let data = std::fs::read_to_string(path)?;

    Ok(data.lines().map(|s| s.to_string()).collect())
}

pub fn load_string<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let data = std::fs::read_to_string(path)?;

    Ok(data)
}

pub fn load_sample_vec(data: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(data.lines().map(|s| s.trim().to_string()).collect())
}
