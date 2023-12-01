use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::Result;


pub fn load_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut f = File::open(path)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
