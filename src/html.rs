use {
    crate::*,
    rustc_hash::FxHashMap,
    scraper::{
        Html,
        Selector,
    },
    std::{
        fs,
        path::{
            Path,
            PathBuf,
        },
    },
};

/// Build the crate documentation in HTML format and return the path to the
///  directory holding all docs (one subdir per crate)
pub fn build_html_doc(crate_path: &Path) -> CradResult<PathBuf> {
    run_cargo_command(&["doc", "--no-deps"], crate_path)?;
    let html_path = crate_path.join("target").join("doc");
    Ok(html_path)
}

/// Extract all links from an HTML file and returns them as a map of link text to URL
pub fn extract_links<P: AsRef<Path>>(path: P) -> CradResult<FxHashMap<String, String>> {
    let path = path.as_ref();
    let html_content = fs::read_to_string(path).map_err(|error| CradError::Read {
        path: path.to_path_buf(),
        error,
    })?;
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
