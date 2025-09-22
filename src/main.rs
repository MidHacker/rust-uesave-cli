use anyhow::{Context, Result};
use std::{env, fs};

fn main() -> Result<()> {
    // Usage: rust-uesave <path-to-active.sav>
    let path = env::args().nth(1).context("missing file path")?;
    let bytes = fs::read(&path)
        .with_context(|| format!("failed to read file: {}", &path))?;

    let value = uesave::to_value(&bytes)
        .context("uesave parse failed")?;

    let out = serde_json::to_string_pretty(&value)?;
    println!("{out}");
    Ok(())
}
