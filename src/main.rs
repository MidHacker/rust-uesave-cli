use anyhow::{Context, Result};
use std::{env, fs};

// NOTE: in uesave 0.6.x the UE4 save type is exposed under `ue4_save::Ue4SaveGame`.
// If your crate exposes it under a slightly different path, the next `use` will fail.
// In that (rare) case, see the fallback import below.
use uesave::ue4_save::Ue4SaveGame;

// ---- If your build still errors on the import above (E0432), uncomment the fallback below
// use uesave::ue4save::Ue4Save as Ue4SaveGame;

fn main() -> Result<()> {
    // Usage: rust-uesave <path-to-active.sav>
    let path = env::args().nth(1).context("missing file path")?;
    let bytes = fs::read(&path)
        .with_context(|| format!("failed to read file: {}", &path))?;

    // Parse Unreal Engine (GVAS) save
    let save: Ue4SaveGame = Ue4SaveGame::from_bytes(&bytes)
        .context("uesave parse failed")?;

    // Convert parsed struct -> JSON
    let json_value = serde_json::to_value(&save)
        .context("convert to JSON failed")?;

    let out = serde_json::to_string_pretty(&json_value)?;
    println!("{out}");
    Ok(())
}
