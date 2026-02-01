use anyhow::{Context, Result};
use indexmap::IndexMap;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Deserialize)]
struct Glossary { meta: Meta, terms: IndexMap<String, Term> }
#[derive(Debug, Deserialize)]
struct Meta { version: String, #[serde(default)] last_reviewed: Option<String> }
#[derive(Debug, Deserialize)]
struct Term {
    title: String, definition: String,
    #[serde(default)] aliases: Vec<String>,
    #[serde(default)] category: Option<String>,
    #[serde(default)] sources: Vec<Source>,
    #[serde(default)] cross_refs: Vec<String>,
    #[serde(default)] notes: Option<String>,
    #[serde(default)] examples: Vec<String>,
    #[serde(default)] status: Option<String>,
    #[serde(default)] since: Option<String>,
    #[serde(default)] version: Option<String>,
}
#[derive(Debug, Deserialize)]
struct Source { citation: String, #[serde(default)] url: Option<String> }

fn slugify(s: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
    re.replace_all(&s.to_lowercase(), "-").trim_matches('-').to_string()
}

fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let yaml = fs::read_to_string(root.join("glossary/glossary.yml"))
        .context("reading glossary/glossary.yml")?;
    let g: Glossary = serde_yaml::from_str(&yaml).context("parsing YAML")?;

    let mut buckets: HashMap<char, Vec<&Term>> = HashMap::new();
    for t in g.terms.values() {
        let ch = t.title.chars().next().unwrap_or('#').to_ascii_uppercase();
        buckets.entry(ch).or_default().push(t);
    }
    let mut letters: Vec<char> = buckets.keys().copied().collect();
    letters.sort_unstable();
    for v in buckets.values_mut() {
        v.sort_by(|a,b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    }

    let mut md = String::new();
    md.push_str("# Glossary\n\n");
    md.push_str(&format!("_Source: `glossary/glossary.yml` • v{}{}\n\n",
        g.meta.version,
        g.meta.last_reviewed.as_deref().map(|d| format!(" • Last reviewed {d}")).unwrap_or_default()
    ));
    if !letters.is_empty() {
        md.push_str("Jump to: ");
        for (i,ch) in letters.iter().enumerate() {
            if i>0 { md.push_str(" · "); }
            md.push_str(&format!("[{0}](#{0})", ch));
        }
        md.push_str("\n\n---\n\n");
    }
    for ch in letters {
        md.push_str(&format!("## {ch}\n\n"));
        for t in buckets.get(&ch).unwrap() {
            let slug = slugify(&t.title);
            md.push_str(&format!("### <a id=\"{slug}\"></a>{}\n\n", t.title));
            md.push_str(&format!("**Definition.** {}\n\n", t.definition));
            if !t.aliases.is_empty() { md.push_str(&format!("**Aliases.** {}\n\n", t.aliases.join(", "))); }
            if let Some(cat)=&t.category { md.push_str(&format!("**Category.** {cat}\n\n")); }
            if !t.cross_refs.is_empty() { md.push_str(&format!("**See also.** {}\n\n", t.cross_refs.join(", "))); }
            if !t.examples.is_empty() {
                md.push_str("**Examples.**\n\n");
                for ex in &t.examples { md.push_str(&format!("- {ex}\n")); }
                md.push_str("\n");
            }
            if let Some(notes)=&t.notes { md.push_str(&format!("**Notes.** {notes}\n\n")); }
            if !t.sources.is_empty() {
                md.push_str("**Sources.**\n\n");
                for s in &t.sources {
                    match &s.url { Some(u)=> md.push_str(&format!("- {} — <{u}>\n", s.citation)),
                                   None   => md.push_str(&format!("- {}\n", s.citation)), }
                }
                md.push_str("\n");
            }
            let mut meta_bits = Vec::new();
            if let Some(v)=&t.version { meta_bits.push(format!("term v{v}")); }
            if let Some(since)=&t.since { meta_bits.push(format!("since {since}")); }
            if let Some(st)=&t.status { if st!="active" { meta_bits.push(format!("status: {st}")); } }
            if !meta_bits.is_empty() { md.push_str(&format!("_{}_\n\n", meta_bits.join(" • "))); }
            md.push_str("[Back to top](#glossary)\n\n---\n\n");
        }
    }
    fs::write(root.join("src/glossary.md"), md).context("writing src/glossary.md")?;
    Ok(())
}
