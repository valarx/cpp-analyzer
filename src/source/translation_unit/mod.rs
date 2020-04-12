pub mod cursor;
pub mod index;

use crate::source::ParsingError;
use clang_sys::*;
pub use cursor::{
    AccessSpecifierType, CodeSpan, ConstructorType, CursorKind, CursorType, Position,
    TemplateArgumentKind, Virtuality,
};
use index::Index;
use libc::c_char;
use std::ffi::CString;
use std::ptr;
use ParsingError::FileNameConversionProblem;

#[derive(Clone, Copy)]
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

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub current_kind: CursorKind,
    pub children: Vec<Entry>,
}

pub struct TU {
    ast: Entry,
}

struct TranslationUnitWrapper {
    pub translation_unit: CXTranslationUnit,
}

fn get_cursor(translation_unit: CXTranslationUnit) -> CXCursor {
    unsafe { clang_getTranslationUnitCursor(translation_unit) }
}

fn get_ast(cursor: CXCursor) -> Entry {
    let mut ast = Entry {
        current_kind: CursorKind::Root,
        children: vec![],
    };
    unsafe {
        clang_visitChildren(
            cursor,
            traverse_cursor,
            &mut ast as *mut _ as *mut std::ffi::c_void,
        );
    }
    ast
}

extern "C" fn traverse_cursor(
    current: CXCursor,
    _parent: CXCursor,
    client_data: *mut core::ffi::c_void,
) -> CXChildVisitResult {
    unsafe {
        if clang_Location_isInSystemHeader(clang_getCursorLocation(current)) == 0 {
            let node = &mut *(client_data as *mut Entry);
            let mut new_node = Entry {
                current_kind: current.into(),
                children: vec![],
            };
            clang_visitChildren(
                current,
                traverse_cursor,
                &mut new_node as *mut _ as *mut std::ffi::c_void,
            );
            node.children.push(new_node);
        } else {
            ()
        }
    }
    CXChildVisit_Continue
}

fn parse_translation_unit(
    index: &Index,
    c_file_name: CString,
    command_line_args_char_vec: Vec<*const c_char>,
    unsaved_files: *mut CXUnsavedFile,
    unsaved_files_num: u32,
    options: &TUOptionsBuilder,
) -> Result<CXTranslationUnit, ParsingError> {
    let mut translation_unit: CXTranslationUnit = ptr::null_mut();
    unsafe {
        let parse_code = clang_parseTranslationUnit2(
            index.index,
            c_file_name.as_ptr(),
            command_line_args_char_vec.as_ptr(),
            command_line_args_char_vec.len() as i32,
            unsaved_files,
            unsaved_files_num,
            options.build(),
            &mut translation_unit,
        );
        match parse_code {
            clang_sys::CXError_Success => {
                assert!(!translation_unit.is_null());
            }
            clang_sys::CXError_Failure => {
                return Err(ParsingError::GenericFailure(
                    c_file_name.into_string().unwrap(),
                ))
            }
            clang_sys::CXError_Crashed => {
                return Err(ParsingError::Crash(c_file_name.into_string().unwrap()))
            }
            clang_sys::CXError_InvalidArguments => {
                return Err(ParsingError::InvalidArguments(
                    c_file_name.into_string().unwrap(),
                ))
            }
            clang_sys::CXError_ASTReadError => {
                return Err(ParsingError::ASTReadError(
                    c_file_name.into_string().unwrap(),
                ))
            }
            _ => return Err(ParsingError::UnknownError(parse_code)),
        };
    }
    Ok(translation_unit)
}

impl TU {
    pub fn new(
        file_name: String,
        index: &Index,
        command_line_args: Vec<String>,
        options: &TUOptionsBuilder,
    ) -> Result<TU, ParsingError> {
        let c_file_name = CString::new(file_name.clone());
        let c_file_name = match c_file_name {
            Ok(value) => value,
            Err(_) => return Err(FileNameConversionProblem(file_name)),
        };
        let command_line_args: Vec<_> = command_line_args
            .into_iter()
            .map(|value| CString::new(value).unwrap())
            .collect();
        let mut command_line_args_char_vec: Vec<*const c_char> = vec![];
        for arg in &command_line_args {
            command_line_args_char_vec.push(arg.as_ptr());
        }
        let unsaved_files: *mut CXUnsavedFile = ptr::null_mut();
        let unsaved_files_num = 0;
        let translation_unit_wrapper = TranslationUnitWrapper {
            translation_unit: parse_translation_unit(
                index,
                c_file_name,
                command_line_args_char_vec,
                unsaved_files,
                unsaved_files_num,
                &options,
            )?,
        };
        let tu = TU {
            ast: get_ast(get_cursor(translation_unit_wrapper.translation_unit)),
        };
        Ok(tu)
    }

    pub fn ast(&self) -> &Entry {
        &self.ast
    }
}

impl Drop for TranslationUnitWrapper {
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
