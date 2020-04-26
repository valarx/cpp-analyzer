use crate::source::compilation_database::Parsed;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Command {
    pub command: String,
    pub file: String,
}

pub fn from_file(file: &Path) -> Result<Vec<Parsed>, Box<dyn Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let commands: Vec<Command> = serde_json::from_reader(reader)?;
    Ok(commands
        .into_iter()
        .map(|val| Parsed {
            args: val
                .command
                .split_whitespace()
                .map(|val| val.to_owned())
                .collect(),
            file: val.file,
        })
        .collect())
}
