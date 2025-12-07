use crate::cache::Cache;

pub fn execute() -> anyhow::Result<()> {
        let cache = if !Cache::path()?.exists() {
                super::update::execute()?
        } else {
                Cache::deserialize()?
        };

        for info in &cache {
                println!("{}  ({})", info.version, info.date);
        }

        Ok(())
}
