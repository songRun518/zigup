use crate::cache::Cache;

pub fn execute(specific: Option<String>) -> anyhow::Result<()> {
        let cache = if !Cache::path()?.exists() {
                super::update::execute()?
        } else {
                Cache::deserialize()?
        };

        if let Some(specific) = specific {
                for version in &cache {
                        if version.version == specific {
                                println!("{}  ({})", version.version, version.date);
                                println!("\nAvailable architecture:");
                                for durl in &version.download_urls {
                                        println!("  {}", durl.arch);
                                }
                                break;
                        }
                }
        } else {
                let width = cache
                        .iter()
                        .map(|v| v.version.len())
                        .max()
                        .expect("Failed to calculate max width of version");
                for version in &cache {
                        println!(
                                "{}{}  ({})",
                                version.version,
                                " ".repeat(width - version.version.len()),
                                version.date
                        );
                }
        }

        Ok(())
}
