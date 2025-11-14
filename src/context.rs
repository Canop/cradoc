use {
    crate::*,
    lazy_regex::*,
    rustdoc_types::{
        Crate,
        ItemEnum,
        Id,
    },
    std::{
        fs::{
            self,
            File,
        },
        io::{
            BufRead,
            BufReader,
            BufWriter,
            Write,
        },
        path::{
            Path,
            PathBuf,
        },
    },
};

pub struct Context {
    pub crate_name: String,
    pub crate_file_name: String,
    pub crate_item_id: Id,
    pub crate_path: PathBuf,
    pub rd_crate: Crate,
    pub html_doc_dir: PathBuf,
    pub online_doc_base_url: String,
}

impl Context {
    pub fn load(
        crate_path: &Path,
    ) -> CradResult<Self> {
        let crate_path = fs::canonicalize(crate_path)?;
        let crate_name = crate_path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .ok_or(CradError::CrateName)?;
        let crate_file_name = crate_name.replace("-", "_");
        let rd_crate = build_and_read_json_doc(&crate_path, &crate_file_name)?;
        let crate_item_id = find_crate_item(&rd_crate)?;
        let online_doc_base_url = format!("https://docs.rs/{}/latest/{}/", crate_name, crate_file_name);
        let html_doc_dir = build_html_doc(&crate_path)?;
        Ok(Self {
            crate_name: crate_name.to_string(),
            crate_file_name,
            crate_item_id,
            crate_path,
            rd_crate,
            html_doc_dir,
            online_doc_base_url,
        })
    }
    pub fn crate_doc_md(
        &self,
    ) -> CradResult<String> {
        let item = self.rd_crate.index.get(&self.crate_item_id)
            .ok_or(CradError::NoCrateItem)?; // Should not happen, by construct
        let Some(docs) = item.docs.as_ref() else {
            return Err(CradError::NoDocInItem);
        };
        let crate_html_path = self.html_doc_dir.join(&self.crate_file_name).join("index.html");
        let links = extract_links(crate_html_path)?;

        // remove single line hidden imports (presumably in code snippets)
        let docs = regex_replace_all!(
            r"(?m)^# use [^\n]+;$\n",
            &docs,
            ""
        );

        // replace code links like [sometype] or [`sometype`] with proper links
        let docs = regex_replace_all!(
            r"\[(`?([\w:]+!?)`?)\]($|[^(])",
            &docs,
            |_, content, text, after| {
                if let Some(link) = links.get(text) {
                    let url = format!("{}{}", self.online_doc_base_url, link);
                    format!("[{content}]({url}){after}")
                } else {
                    warn!("no link found for `{text}`");
                    format!("{content}{after}")
                }
            }
        );
        Ok(docs.to_string())
    }
    pub fn update_all_md_files(
        &self,
    ) -> CradResult<()> {
        // let's start with only the root file until I decide to manage .gitignore, etc.
        for entry in fs::read_dir(&self.crate_path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if !file_type.is_file() {
                continue;
            }
            let file_name = entry.file_name();
            let Some(file_name) = file_name.to_str() else {
                continue; // too weird for us
            };
            if ! regex_is_match!(r"\.md$"i, file_name) {
                continue;
            }
            let path = entry.path();
            if self.update_file(&path)? {
                println!("Updated crate doc in file {:?}", path);
            }
        }
        Ok(())
    }
    /// Look in the file for `<!-- cradoc -->` marker, or the couple of tags for an
    /// existing span, and replace the content by the generated crate doc markdown.
    ///
    /// Return true when the file was changed.
    pub fn update_file(
        &self,
        file_path: &Path,
    ) -> CradResult<bool> {
        // load the whole file in memory, in lines
        let mut input_lines = Vec::new();
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            input_lines.push(line?);
        }

        // do the change(s)
        let mut new = 0;
        let mut updated = 0;
        let mut output = Vec::new(); // not always "lines"
        let mut replacing = false;
        for line in input_lines {
            if replacing {
                if regex_is_match!(r"^\s*<!--\s+cradoc\s+end\s+-->\s*$", &line) {
                    replacing = false;
                    output.push(line);
                }
            } else {
                if regex_is_match!(r"^\s*<!--\s+cradoc\s+start\s+-->\s*$", &line) {
                    output.push(line);
                    let doc = self.crate_doc_md()?;
                    output.push(doc);
                    replacing = true;
                    updated += 1;
                } else if regex_is_match!(r"^\s*<!--\s+cradoc\s+-->\s*$", &line) {
                    output.push("<!-- cradoc start -->\n".to_string());
                    let doc = self.crate_doc_md()?;
                    output.push(doc);
                    output.push("<!-- cradoc end -->\n".to_string());
                    new += 1;
                } else {
                    output.push(line);
                }
            }
        }
        if new + updated == 0 {
            return Ok(false);
        }

        // rewrite the file
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        let mut file = BufWriter::new(file);
        for part in output {
            writeln!(file, "{}", part)?;
        }
        Ok(true)
    }
}

pub fn find_crate_item(
    rd_crate: &Crate,
) -> CradResult<Id> {
    for item in rd_crate.index.values() {
        let ItemEnum::Module(module) = &item.inner else {
            continue;
        };
        if module.is_crate {
            return Ok(item.id);
        }
    }
    Err(CradError::NoCrateItem)
}
