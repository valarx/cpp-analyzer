pub mod index;

use crate::source::ParsingError;
use clang_sys::*;
use index::Index;
use libc::c_char;
use std::ffi::CString;
use std::ptr;

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

pub struct TU {
    pub translation_unit: CXTranslationUnit,
}

impl TU {
    pub fn new(
        file_name: String,
        index: &Index,
        options: TUOptionsBuilder,
    ) -> Result<TU, ParsingError> {
        let c_file_name = CString::new(file_name);
        let c_file_name = match c_file_name {
            Ok(value) => value,
            Err(_) => return Err(ParsingError::FileNameConversionProblem),
        };
        unsafe {
            let mut result = TU {
                translation_unit: ptr::null_mut(),
            };
            let command_line_args: *const *const c_char = ptr::null(); // FIXME
            let command_line_args_num = 0;
            let unsaved_files: *mut CXUnsavedFile = ptr::null_mut();
            let unsaved_files_num = 0;
            let parse_code = clang_parseTranslationUnit2(
                index.index,
                c_file_name.as_ptr(),
                command_line_args,
                command_line_args_num,
                unsaved_files,
                unsaved_files_num,
                options.build(),
                &mut result.translation_unit,
            );

            match parse_code {
                clang_sys::CXError_Success => {
                    assert!(!result.translation_unit.is_null());
                    return Ok(result);
                }
                clang_sys::CXError_Failure => return Err(ParsingError::GenericFailure),
                clang_sys::CXError_Crashed => return Err(ParsingError::Crash),
                clang_sys::CXError_InvalidArguments => return Err(ParsingError::InvalidArguments),
                clang_sys::CXError_ASTReadError => return Err(ParsingError::ASTReadError),
                _ => return Err(ParsingError::UnknownError(parse_code)),
            };
        }
    }
}

impl Drop for TU {
    fn drop(&mut self) {
        unsafe {
            clang_disposeTranslationUnit(self.translation_unit);
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
