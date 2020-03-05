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
    FileNameConversionProblem,
    IndexCreationFailure,
    GenericFailure,
    Crash,
    InvalidArguments,
    ASTReadError,
    UnknownError(i32),
}

pub struct Source {
    index: Index,
    pub translation_units: Vec<TU>,
}

impl Source {
    pub fn new(
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
        let translation_unit = TU::new(file_name, &result.index, command_line_args, options)?;
        result.translation_units.push(translation_unit);
        Ok(result)
    }
}
