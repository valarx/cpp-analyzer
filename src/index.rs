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

pub struct TUOptionsBuilder {
    resulting_options: i32,
}

impl TUOptionsBuilder {
    pub fn new() -> TUOptionsBuilder {
        TUOptionsBuilder {
            resulting_options: 0,
        }
    }

    pub fn detailed_preprocessor_record(&mut self) -> &mut TUOptionsBuilder {
        self.resulting_options |= CXTranslationUnit_DetailedPreprocessingRecord;
        self
    }

    pub fn incomplete(&mut self) -> &mut TUOptionsBuilder {
        self.resulting_options |= CXTranslationUnit_Incomplete;
        self
    }

    pub fn precompiled_preamble(&mut self) -> &mut TUOptionsBuilder {
        self.resulting_options |= CXTranslationUnit_PrecompiledPreamble;
        self
    }
    pub fn cache_completion_results(&mut self) -> &mut TUOptionsBuilder {
        self.resulting_options |= CXTranslationUnit_CacheCompletionResults;
        self
    }

    pub fn for_serialization(&mut self) -> &mut TUOptionsBuilder {
        self.resulting_options |= CXTranslationUnit_ForSerialization;
        self
    }

    pub fn chained_phc(&mut self) -> &mut TUOptionsBuilder {
        self.resulting_options |= CXTranslationUnit_CXXChainedPCH;
        self
    }

    pub fn skip_function_bodies(&mut self) -> &mut TUOptionsBuilder {
        self.resulting_options |= CXTranslationUnit_SkipFunctionBodies;
        self
    }

    pub fn include_brief_comments_in_code_completion(&mut self) -> &mut TUOptionsBuilder {
        self.resulting_options |= CXTranslationUnit_IncludeBriefCommentsInCodeCompletion;
        self
    }

    pub fn build(&self) -> i32 {
        self.resulting_options
    }
}

impl Index {
    pub fn new(
        file_name: String,
        phc_mode: DeclarationFromPHCMode,
        diagnostics_mode: DiagnosticsMode,
        options: TUOptionsBuilder,
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
            let translation_unit = clang_parseTranslationUnit(
                index,
                c_file_name.as_ptr(),
                command_line_args,
                command_line_args_num,
                unsaved_files,
                unsaved_files_num,
                options.build(),
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
    use super::*;
    #[test]
    fn test_options_builder() {
        let mut tu_options = TUOptionsBuilder::new();
        assert_eq!(tu_options.build(), CXTranslationUnit_None);
        tu_options.detailed_preprocessor_record();
        assert_eq!(
            tu_options.build(),
            CXTranslationUnit_DetailedPreprocessingRecord
        );
        tu_options.incomplete();
        assert_eq!(
            tu_options.build(),
            CXTranslationUnit_DetailedPreprocessingRecord | CXTranslationUnit_Incomplete
        );
        tu_options.precompiled_preamble();
        assert_eq!(
            tu_options.build(),
            CXTranslationUnit_DetailedPreprocessingRecord
                | CXTranslationUnit_Incomplete
                | CXTranslationUnit_PrecompiledPreamble
        );
        tu_options
            .cache_completion_results()
            .for_serialization()
            .chained_phc()
            .skip_function_bodies()
            .include_brief_comments_in_code_completion();
        assert_eq!(
            tu_options.build(),
            CXTranslationUnit_DetailedPreprocessingRecord
                | CXTranslationUnit_Incomplete
                | CXTranslationUnit_PrecompiledPreamble
                | CXTranslationUnit_CacheCompletionResults
                | CXTranslationUnit_ForSerialization
                | CXTranslationUnit_CXXChainedPCH
                | CXTranslationUnit_SkipFunctionBodies
                | CXTranslationUnit_IncludeBriefCommentsInCodeCompletion
        );
    }
}
