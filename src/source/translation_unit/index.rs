use crate::source::ParsingError;
use clang_sys::*;
use std::ptr;

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

impl Index {
    pub fn new(
        phc_mode: DeclarationFromPHCMode,
        diagnostics_mode: DiagnosticsMode,
    ) -> Result<Index, ParsingError> {
        let mut result = Index {
            index: ptr::null_mut(),
        };
        unsafe {
            result.index = clang_createIndex(phc_mode as i32, diagnostics_mode as i32);
        }
        if result.index.is_null() {
            return Err(ParsingError::IndexCreationFailure);
        }
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
