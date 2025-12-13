use colored::Colorize;

use crate::config::Config;

pub fn execute(version: Option<String>) {
    let mut config = Config::load();
    if config.versions_info.is_none() {
        config = super::update::execute();
    }

    let versions_info = config.versions_info.clone().unwrap();

    if let Some(specific) = version {
        for version in &versions_info {
            if version.version == specific {
                println!("{}  ({})", version.version.cyan().bold(), version.date);
                println!("\n{}", "Available architecture:".bold().underline());
                for du in &version.arch_specific {
                    println!("  {}", du.arch.italic().purple());
                }
                break;
            }
        }
    } else {
        for version in &versions_info {
            println!("{}    ({})", version.version.cyan().bold(), version.date);
        }
    }
}
