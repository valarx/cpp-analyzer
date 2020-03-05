use crate::source::ParsingError;
use clang_sys::*;

pub enum DeclarationFromPHCMode {
    Include = 0,
    Exclude = 1,
}

pub enum DiagnosticsMode {
    Disabled = 0,
    Enabled = 1,
}

pub struct Index {
    pub index: CXIndex,
}

fn create_index(
    phc_mode: DeclarationFromPHCMode,
    diag_mode: DiagnosticsMode,
) -> Result<CXIndex, ParsingError> {
    unsafe {
        let res = clang_createIndex(phc_mode as i32, diag_mode as i32);
        if res.is_null() {
            return Err(ParsingError::IndexCreationFailure);
        } else {
            Ok(res)
        }
    }
}

impl Index {
    pub fn new(
        phc_mode: DeclarationFromPHCMode,
        diagnostics_mode: DiagnosticsMode,
    ) -> Result<Index, ParsingError> {
        let result = Index {
            index: create_index(phc_mode, diagnostics_mode)?,
        };
        Ok(result)
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            clang_disposeIndex(self.index);
        }
    }
}
