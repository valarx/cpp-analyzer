use clang_sys::*;
use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

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

struct Index {
    pub index: CXIndex,
}

impl Index {
    fn new(
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
        assert!(!self.index.is_null());
        unsafe {
            clang_disposeIndex(self.index);
        }
    }
}

impl Source {
    pub fn new(
        file_name: String,
        phc_mode: DeclarationFromPHCMode,
        diagnostics_mode: DiagnosticsMode,
        options: TUOptionsBuilder,
    ) -> Result<Source, ParsingError> {
        let c_file_name = CString::new(file_name);
        let c_file_name = match c_file_name {
            Ok(value) => value,
            Err(_) => return Err(ParsingError::FileNameConversionProblem),
        };
        unsafe {
            let index = Index::new(phc_mode, diagnostics_mode)?;

            let command_line_args: *const *const c_char = ptr::null(); // FIXME
            let command_line_args_num = 0;
            let unsaved_files: *mut CXUnsavedFile = ptr::null_mut();
            let unsaved_files_num = 0;
            let mut translation_unit: CXTranslationUnit = ptr::null_mut();
            let parse_code = clang_parseTranslationUnit2(
                index.index,
                c_file_name.as_ptr(),
                command_line_args,
                command_line_args_num,
                unsaved_files,
                unsaved_files_num,
                options.build(),
                &mut translation_unit,
            );

            match parse_code {
                clang_sys::CXError_Success => (),
                clang_sys::CXError_Failure => return Err(ParsingError::GenericFailure),
                clang_sys::CXError_Crashed => return Err(ParsingError::Crash),
                clang_sys::CXError_InvalidArguments => return Err(ParsingError::InvalidArguments),
                clang_sys::CXError_ASTReadError => return Err(ParsingError::ASTReadError),
                _ => return Err(ParsingError::UnknownError(parse_code)),
            };
            assert!(
                !translation_unit.is_null(),
                "Could not parse translation unit"
            );

            let cursor = clang_getTranslationUnitCursor(translation_unit);
            let mut result: Source = Source {
                cursor_data: vec![],
            };
            clang_visitChildren(
                cursor,
                traverse_cursor,
                &mut result as *mut _ as *mut std::ffi::c_void,
            );

            clang_disposeTranslationUnit(translation_unit);
            Ok(result)
        }
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
