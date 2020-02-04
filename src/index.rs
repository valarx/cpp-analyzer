use clang_sys::*;
use libc::c_char;
use std::ffi::CString;
use std::ptr;

pub struct Index {}

pub enum DeclarationFromPHCMode {
    Include = 0,
    Exclude = 1,
}

pub enum DiagnosticsMode {
    Disabled = 0,
    Enabled = 1,
}

impl Index {
    pub fn new(
        file_name: String,
        phc_mode: DeclarationFromPHCMode,
        diagnostics_mode: DiagnosticsMode,
    ) -> Index {
        // TODO error handling
        unsafe {
            let index = clang_createIndex(phc_mode as i32, diagnostics_mode as i32);
            assert!(!index.is_null(), "Could not create clang index");

            let c_file_name =
                CString::new(file_name).expect("Failed to convert file name to c string");
            let command_line_args: *const *const c_char = ptr::null(); // FIXME
            let command_line_args_num = 0;
            let unsaved_files: *mut CXUnsavedFile = ptr::null_mut();
            let unsaved_files_num = 0;
            let options = 0; // TODO do something about it
            let translation_unit = clang_parseTranslationUnit(
                index,
                c_file_name.as_ptr(),
                command_line_args,
                command_line_args_num,
                unsaved_files,
                unsaved_files_num,
                options,
            );
            assert!(
                !translation_unit.is_null(),
                "Could not parse translation unit"
            );
            clang_disposeIndex(index);
            clang_disposeTranslationUnit(translation_unit);
        }
        Index {}
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn create_index() {
        // let _index = Index::new(DeclarationFromPHCMode::Exclude, DiagnosticsMode::Enabled);
    }
}
