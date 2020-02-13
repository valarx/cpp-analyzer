mod translation_unit;

use translation_unit::index::Index;
pub use translation_unit::AccessSpecifierType;
pub use translation_unit::CursorKind;
pub use translation_unit::TUOptionsBuilder;
use translation_unit::TU;

pub enum DeclarationFromPHCMode {
    Include = 0,
    Exclude = 1,
}

pub enum DiagnosticsMode {
    Disabled = 0,
    Enabled = 1,
}

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
