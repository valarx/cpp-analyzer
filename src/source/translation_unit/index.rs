use crate::source::compiler_instance::CompilerInstance;
use crate::source::ParsingError;
use clang_sys::*;

#[derive(Clone, Copy)]
pub enum DeclarationFromPHCMode {
    Include = 0,
    Exclude = 1,
}

#[derive(Clone, Copy)]
pub enum DiagnosticsMode {
    Disabled = 0,
    Enabled = 1,
}

pub struct Index {
    pub index: CXIndex,
    _compiler: CompilerInstance,
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
        compiler: CompilerInstance,
    ) -> Result<Index, ParsingError> {
        let result = Index {
            index: create_index(phc_mode, diagnostics_mode)?,
            _compiler: compiler,
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
