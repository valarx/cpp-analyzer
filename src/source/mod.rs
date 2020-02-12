mod translation_unit;

use clang_sys::*;
use std::ffi::CStr;
use translation_unit::index::Index;
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
    pub cursor_data: Vec<String>,
}

extern "C" fn traverse_cursor(
    current: CXCursor,
    _parent: CXCursor,
    client_data: *mut core::ffi::c_void,
) -> CXChildVisitResult {
    unsafe {
        let index = &mut *(client_data as *mut Source);
        let cursor_spelling = clang_getCursorSpelling(current);
        let _cursor_kind_spelling = clang_getCursorKindSpelling(clang_getCursorKind(current));
        let cursor_spelling_as_string = clang_getCString(cursor_spelling);
        let cursor_spelling_as_string = CStr::from_ptr(cursor_spelling_as_string)
            .to_string_lossy()
            .into_owned();
        index.cursor_data.push(cursor_spelling_as_string);
        clang_disposeString(cursor_spelling);
    }
    CXChildVisit_Recurse
}

impl Source {
    pub fn new(
        file_name: String,
        phc_mode: DeclarationFromPHCMode,
        diagnostics_mode: DiagnosticsMode,
        options: TUOptionsBuilder,
    ) -> Result<Source, ParsingError> {
        let index = Index::new(phc_mode, diagnostics_mode)?;
        let translation_unit = TU::new(file_name, &index, options)?;
        let mut result: Source = Source {
            cursor_data: vec![],
        };
        unsafe {
            let cursor = clang_getTranslationUnitCursor(translation_unit.translation_unit);
            clang_visitChildren(
                cursor,
                traverse_cursor,
                &mut result as *mut _ as *mut std::ffi::c_void,
            );
        }
        Ok(result)
    }
}
