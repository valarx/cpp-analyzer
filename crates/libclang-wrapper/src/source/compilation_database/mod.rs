mod json_compile_commands;

use std::error::Error;
use std::path::Path;

#[derive(PartialEq, Debug)]
pub struct Parsed {
    pub args: Vec<String>,
    pub file: String,
}

pub struct CompilationDatabase {
    pub commands: Vec<Parsed>,
}

impl CompilationDatabase {
    pub fn new(file: &Path) -> Result<CompilationDatabase, Box<dyn Error>> {
        Ok(CompilationDatabase {
            commands: json_compile_commands::from_file(file)?,
        })
    }
}
