use {
    crate::*,
    scraper::{
        Html,
        Selector,
    },
    rustc_hash::FxHashMap,
    std::{
        fs,
        path::Path,
        path::PathBuf,
        process::{
            Command,
        },
    },
};

/// Build the crate documentation in HTML format and return the path to the
///  directory holding all docs (one subdir per crate)
pub fn build_html_doc(
    crate_path: &Path,
) -> CradResult<PathBuf> {
    let exe = "cargo";
    let args = [
        "doc",
    ];
    let output = Command::new(exe)
        .args(args)
        .current_dir(&crate_path)
        .output()?;
    // TODO check output.status
    let html_path = crate_path
        .join("target")
        .join("doc");
    Ok(html_path)
}

/// Extract all links from an HTML file and returns them as a map of link text to URL
pub fn extract_links<P: AsRef<Path>>(path: P) -> CradResult<FxHashMap<String, String>> {
    let html_content = fs::read_to_string(path)?;
    let document = Html::parse_document(&html_content);
    let selector = Selector::parse("a").unwrap(); // safety: to be checked
    let mut links = FxHashMap::default();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            let text = element.text().collect::<String>();
            links.insert(text.trim().to_string(), href.to_string());
        }
    }
    Ok(links)
}
