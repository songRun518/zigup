use anyhow::Context;
use reqwest::blocking;
use scraper::{Html, Selector};

pub const ZIG_DOWNLOAD_PAGE: &str = "https://ziglang.org/download/";

pub fn execute() -> anyhow::Result<()> {
    let response = blocking::get(ZIG_DOWNLOAD_PAGE).context("Failed to get zig download page")?;

    let page = Html::parse_document(
        &response
            .text()
            .context("Failed to get zig download page text")?,
    );

    let version_list = page
        .select(&Selector::parse("body div#content div.container div").unwrap())
        .next_back()
        .unwrap();

    let versions = version_list
        .select(&Selector::parse("h2").unwrap())
        .map(|ele| ele.inner_html())
        .collect::<Vec<_>>();

    let times = version_list
        .select(&Selector::parse("ul").unwrap())
        .map(|ul| {
            ul.select(&Selector::parse("li").unwrap())
                .next()
                .unwrap()
                .inner_html()
        })
        .collect::<Vec<_>>();

    // let cache_dir = std::env::current_dir()
    //     .context("Failed to get current dir")?
    //     .join(".zigup");

    // if !cache_dir.exists() {
    //     std::fs::create_dir_all(&cache_dir).context("Failed to create cache dir")?;
    // }

    Ok(())
}
