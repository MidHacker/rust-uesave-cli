use anyhow::{Context, Result};
use std::{env, fs::File};
use uesave::Save;

fn main() -> Result<()> {
    // Usage: rust-uesave <path-to-active.sav>
    let path = env::args().nth(1).context("missing file path")?;

    // Read + parse Unreal Engine save
    let mut f = File::open(&path)
        .with_context(|| format!("failed to open file: {}", &path))?;
    let save = Save::read(&mut f)
        .context("uesave parse failed")?;

    // Convert parsed struct -> JSON
    let json = serde_json::to_string_pretty(&save)
        .context("convert to JSON failed")?;

    println!("{json}");
    Ok(())
}
