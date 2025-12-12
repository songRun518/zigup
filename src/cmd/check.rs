use colored::Colorize;

use crate::cache::Cache;

pub fn execute(specific: Option<String>) {
        let cache = if !Cache::path().exists() {
                super::update::execute()
        } else {
                Cache::deserialize()
        };

        if let Some(specific) = specific {
                for version in &cache {
                        if version.version == specific {
                                println!("{}  ({})", version.version.bold().cyan(), version.date);
                                println!("\n{}", "Available architecture:".bold().underline());
                                for du in &version.download_urls {
                                        println!("  {}", du.arch.italic().purple());
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
                                version.version.bold().cyan(),
                                " ".repeat(width - version.version.len()),
                                version.date
                        );
                }
        }
}
