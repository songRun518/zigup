use anyhow::Context;

pub fn execute() -> anyhow::Result<()> {
    let cache_dir = std::env::current_dir()
        .context("Failed to get current dir")?
        .join(".zigup.ron");

    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).context("Failed to create cache dir")?;
    }

    Ok(())
}
