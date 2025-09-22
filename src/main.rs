use anyhow::{Context, Result};
use std::{env, fs};
use uesave::ue4_save::Ue4SaveGame;

fn main() -> Result<()> {
    // Usage: rust-uesave <path-to-active.sav>
    let path = env::args().nth(1).context("missing file path")?;
    let bytes = fs::read(&path)
        .with_context(|| format!("failed to read file: {}", &path))?;

    // Parse Unreal Engine save file
    let save = Ue4SaveGame::from_bytes(&bytes)
        .context("uesave parse failed")?;

    // Convert parsed struct -> JSON
    let value = serde_json::to_value(&save)
        .context("convert to JSON failed")?;

    let out = serde_json::to_string_pretty(&value)?;
    println!("{out}");
    Ok(())
}
