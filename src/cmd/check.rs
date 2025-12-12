use colored::Colorize;

use crate::cache;

pub fn execute(specific: Option<String>) {
    let cache = if !cache::path().exists() {
        super::update::execute()
    } else {
        cache::deserialize()
    };

    if let Some(specific) = specific {
        for version in &cache {
            if version.version == specific {
                println!("{}  ({})", version.version.bold().cyan(), version.date);
                println!("\n{}", "Available architecture:".bold().underline());
                for du in &version.arch_and_url {
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
