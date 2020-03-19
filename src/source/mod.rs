mod compilation_database;
mod translation_unit;

pub use compilation_database::{CompilationDatabase, Parsed};
use translation_unit::index::Index;
pub use translation_unit::index::{DeclarationFromPHCMode, DiagnosticsMode};
pub use translation_unit::TUOptionsBuilder;
use translation_unit::TU;
pub use translation_unit::{
    AccessSpecifierType, CodeSpan, ConstructorType, CursorKind, CursorType, Entry, Position,
    TemplateArgumentKind, Virtuality,
};

#[derive(Debug)]
pub enum ParsingError {
    FileNameConversionProblem(String),
    IndexCreationFailure,
    GenericFailure(String),
    Crash(String),
    InvalidArguments(String),
    ASTReadError(String),
    UnknownError(i32),
}

pub struct Source {
    index: Index,
    pub translation_units: Vec<Result<TU, ParsingError>>,
}

impl Source {
    pub fn from_file(
        file_name: String,
        phc_mode: DeclarationFromPHCMode,
        diagnostics_mode: DiagnosticsMode,
        command_line_args: Vec<String>,
        options: TUOptionsBuilder,
    ) -> Result<Source, ParsingError> {
        let mut result = Source {
            index: Index::new(phc_mode, diagnostics_mode)?,
            translation_units: vec![],
        };
        result.translation_units.push(TU::new(
            file_name,
            &result.index,
            command_line_args,
            &options,
        ));
        Ok(result)
    }

    pub fn from_compilation_database(
        phc_mode: DeclarationFromPHCMode,
        diagnostics_mode: DiagnosticsMode,
        compilation_database: CompilationDatabase,
        options: TUOptionsBuilder,
    ) -> Result<Source, ParsingError> {
        let index = Index::new(phc_mode, diagnostics_mode)?;
        let mut result = Source {
            index,
            translation_units: vec![],
        };
        result.translation_units = compilation_database
            .commands
            .into_iter()
            .map(|val| TU::new(val.file, &result.index, val.args, &options))
            .collect();
        Ok(result)
    }
}
