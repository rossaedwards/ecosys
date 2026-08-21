use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "fuxyez.pest"]
pub struct FuxParser;

#[derive(Debug)]
pub struct FuxFile {
    pub sigils: Vec<SigilDef>,
    pub rituals: Vec<RitualDef>,
}

pub fn read_fux(path: impl AsRef<std::path::Path>) -> Result<FuxFile, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let pairs = FuxParser::parse(Rule::file, &content)?;
    // Parse logic...
    Ok(FuxFile { sigils: vec![], rituals: vec![] })
}
