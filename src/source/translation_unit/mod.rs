pub mod index;

use crate::source::ParsingError;
use clang_sys::*;
use index::Index;
use libc::c_char;
use std::ffi::{CStr, CString};
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

#[derive(Debug, PartialEq)]
pub enum AccessSpecifierType {
    Invalid,
    Public,
    Protected,
    Private,
}

#[derive(Debug, PartialEq)]
pub enum CursorKind {
    Unknown(String),
    Struct(String, AccessSpecifierType),
    Union(String, AccessSpecifierType),
    Class(String, AccessSpecifierType),
    Field(String, AccessSpecifierType),
    Enum(String, AccessSpecifierType),
    EnumConstant(String),
    Function(String),
    Variable(String),
    Parameter(String),
    Typedef(String, AccessSpecifierType),
    Method(String, AccessSpecifierType),
    Namespace(String),
    LinkageSpec(String),
    Constructor(String, AccessSpecifierType),
    Destructor(String, AccessSpecifierType),
    ConversionFunction(String, AccessSpecifierType),
    TemplateTypeParameter(String),
    TemplateNonTypeParameter(String),
    TemplateTemplateParameter(String),
    FunctionTemplate(String),
    ClassTemplate(String),
    ClassTemplatePartial(String),
    NamespaceAlias(String),
    UsingDirective(String),
    TypeAlias(String, AccessSpecifierType),
    AccessSpecifier(AccessSpecifierType),
    TypeReference(String),
    BaseSpecifier(String),
    NotSupported(String, i32),
}

pub struct TU {
    pub translation_unit: CXTranslationUnit,
    cursors: Vec<CursorKind>,
}

fn get_access_specifier(cursor: CXCursor) -> AccessSpecifierType {
    unsafe {
        let access_specifier_type = clang_getCXXAccessSpecifier(cursor);
        match access_specifier_type {
            clang_sys::CX_CXXPrivate => AccessSpecifierType::Private,
            clang_sys::CX_CXXProtected => AccessSpecifierType::Protected,
            clang_sys::CX_CXXPublic => AccessSpecifierType::Public,
            _ => AccessSpecifierType::Invalid,
        }
    }
}

fn map_cursor_kind(cursor: CXCursor, clang_kind: i32, spelling: String) -> CursorKind {
    match clang_kind {
        clang_sys::CXCursor_UnexposedDecl => CursorKind::Unknown(spelling),
        clang_sys::CXCursor_StructDecl => {
            CursorKind::Struct(spelling, get_access_specifier(cursor))
        }
        clang_sys::CXCursor_UnionDecl => CursorKind::Union(spelling, get_access_specifier(cursor)),
        clang_sys::CXCursor_ClassDecl => CursorKind::Class(spelling, get_access_specifier(cursor)),
        clang_sys::CXCursor_FieldDecl => CursorKind::Field(spelling, get_access_specifier(cursor)),
        clang_sys::CXCursor_EnumDecl => CursorKind::Enum(spelling, get_access_specifier(cursor)),
        clang_sys::CXCursor_EnumConstantDecl => CursorKind::EnumConstant(spelling),
        clang_sys::CXCursor_FunctionDecl => CursorKind::Function(spelling),
        clang_sys::CXCursor_VarDecl => CursorKind::Variable(spelling),
        clang_sys::CXCursor_ParmDecl => CursorKind::Parameter(spelling),
        clang_sys::CXCursor_TypedefDecl => {
            CursorKind::Typedef(spelling, get_access_specifier(cursor))
        }
        clang_sys::CXCursor_CXXMethod => CursorKind::Method(spelling, get_access_specifier(cursor)),
        clang_sys::CXCursor_Namespace => CursorKind::Namespace(spelling),
        clang_sys::CXCursor_LinkageSpec => CursorKind::LinkageSpec(spelling),
        clang_sys::CXCursor_Constructor => {
            CursorKind::Constructor(spelling, get_access_specifier(cursor))
        }
        clang_sys::CXCursor_Destructor => {
            CursorKind::Destructor(spelling, get_access_specifier(cursor))
        }
        clang_sys::CXCursor_ConversionFunction => {
            CursorKind::ConversionFunction(spelling, get_access_specifier(cursor))
        }
        clang_sys::CXCursor_TemplateTypeParameter => CursorKind::TemplateTypeParameter(spelling),
        clang_sys::CXCursor_NonTypeTemplateParameter => {
            CursorKind::TemplateNonTypeParameter(spelling)
        }
        clang_sys::CXCursor_TemplateTemplateParameter => {
            CursorKind::TemplateTemplateParameter(spelling)
        }
        clang_sys::CXCursor_FunctionTemplate => CursorKind::FunctionTemplate(spelling),
        clang_sys::CXCursor_ClassTemplate => CursorKind::ClassTemplate(spelling),
        clang_sys::CXCursor_ClassTemplatePartialSpecialization => {
            CursorKind::ClassTemplatePartial(spelling)
        }
        clang_sys::CXCursor_NamespaceAlias => CursorKind::NamespaceAlias(spelling),
        clang_sys::CXCursor_UsingDirective => CursorKind::UsingDirective(spelling),
        clang_sys::CXCursor_TypeAliasDecl => {
            CursorKind::TypeAlias(spelling, get_access_specifier(cursor))
        }
        clang_sys::CXCursor_CXXAccessSpecifier => {
            CursorKind::AccessSpecifier(get_access_specifier(cursor))
        }
        clang_sys::CXCursor_TypeRef => CursorKind::TypeReference(spelling),
        clang_sys::CXCursor_CXXBaseSpecifier => CursorKind::BaseSpecifier(spelling),
        _ => CursorKind::NotSupported(spelling, clang_kind),
    }
}

extern "C" fn traverse_cursor(
    current: CXCursor,
    _parent: CXCursor,
    client_data: *mut core::ffi::c_void,
) -> CXChildVisitResult {
    unsafe {
        let translation_unit = &mut *(client_data as *mut TU);
        let cursor_spelling = clang_getCursorSpelling(current);
        let cursor_kind = clang_getCursorKind(current);

        let cursor_spelling_as_string = clang_getCString(cursor_spelling);
        let cursor_spelling_as_string = CStr::from_ptr(cursor_spelling_as_string)
            .to_string_lossy()
            .into_owned();

        translation_unit.cursors.push(map_cursor_kind(
            current,
            cursor_kind,
            cursor_spelling_as_string,
        ));
        clang_disposeString(cursor_spelling);
    }
    CXChildVisit_Recurse
}

impl TU {
    pub fn new(
        file_name: String,
        index: &Index,
        command_line_args: Vec<String>,
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
                cursors: vec![],
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
            let parse_code = clang_parseTranslationUnit2(
                index.index,
                c_file_name.as_ptr(),
                command_line_args_char_vec.as_ptr(),
                command_line_args_char_vec.len() as i32,
                unsaved_files,
                unsaved_files_num,
                options.build(),
                &mut result.translation_unit,
            );

            match parse_code {
                clang_sys::CXError_Success => {
                    assert!(!result.translation_unit.is_null());
                }
                clang_sys::CXError_Failure => return Err(ParsingError::GenericFailure),
                clang_sys::CXError_Crashed => return Err(ParsingError::Crash),
                clang_sys::CXError_InvalidArguments => return Err(ParsingError::InvalidArguments),
                clang_sys::CXError_ASTReadError => return Err(ParsingError::ASTReadError),
                _ => return Err(ParsingError::UnknownError(parse_code)),
            };

            let cursor = clang_getTranslationUnitCursor(result.translation_unit);
            clang_visitChildren(
                cursor,
                traverse_cursor,
                &mut result as *mut _ as *mut std::ffi::c_void,
            );

            Ok(result)
        }
    }

    pub fn get_cursors(&self) -> &Vec<CursorKind> {
        &self.cursors
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
