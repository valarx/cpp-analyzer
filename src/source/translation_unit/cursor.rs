use clang_sys::*;
use std::ffi::CStr;
use std::ptr;

#[derive(Debug, PartialEq)]
pub enum AccessSpecifierType {
    Invalid,
    Public,
    Protected,
    Private,
}

#[derive(Debug, PartialEq)]
pub enum CursorType {
    Unexposed,
    Void,
    Bool,
    CharU, // implementation defined
    UnsignedChar,
    Char16,
    Char32,
    UnsignedShort,
    UnsignedInt,
    UnsignedLong,
    UnsignedLongLong,
    UnsignedInt128,
    CharS, // implementation defined
    SignedChar,
    WideChar,
    Short,
    Int,
    Long,
    LongLong,
    Int128,
    Float,
    Double,
    LongDouble,
    Nullptr,
    Overload,
    Dependent, // TODO what is this?
    Float128,
    Half, // TODO what is this?
    Float16,
    ShortAccum, // TODO what is this?
    Accum,
    LongAccum,
    UnsignedShortAccum,
    UnsignedAccum,
    UnsignedLongAccum,
    Complex,
    Pointer,
    BlockPointer, // TODO what is this?
    LValueReference,
    RValueReference,
    Record, // TODO what is this?
    Enum,
    Typedef,
    FunctionNoProto, // TODO what is this?
    FunctionProto,
    ConstantArray,       // TODO what is this?
    Vector,              // TODO what is this?
    IncompleteArray,     // TODO what is this?
    VariableArray,       // TODO what is this?
    DependentSizedArray, // TODO what is this?
    MemberPointer,
    Auto,
    Elaborated, // TODO check this
    Pipe,       // TODO what is this?
    Attributed, // TODO what is this?
    Invalid,
    NotSupported(i32),
}

#[derive(Debug, PartialEq)]
pub enum TemplateArgumentKind {
    Null,
    Type,
    Declaration,
    NullPtr,
    Integral,
    Template,
    TemplateExpansion,
    Expression,
    Pack,
    Invalid,
}

#[derive(Debug, PartialEq)]
pub enum ConstructorType {
    None,
    Converting,
    Copy,
    Default,
    Move,
}

#[derive(Debug, PartialEq)]
pub struct Position {
    pub file_name: String,
    pub line: u32,
    pub col: u32,
}
#[derive(Debug, PartialEq)]
pub struct CodeSpan {
    pub start_pos: Position,
    pub end_pos: Position,
}

#[derive(Debug, PartialEq)]
pub enum Virtuality {
    NonVirtual,
    PureVirtual,
    Virtual,
    Static,
}

#[derive(Debug, PartialEq)]
pub enum CursorKind {
    Unexposed(String),
    Struct(String, CodeSpan, AccessSpecifierType),
    Union(String, CodeSpan, AccessSpecifierType),
    Class(String, CodeSpan, AccessSpecifierType),
    Field(String, CodeSpan, AccessSpecifierType, CursorType),
    Enum(String, CodeSpan, AccessSpecifierType),
    EnumConstant(String, CodeSpan),
    Function {
        spelling: String,
        code_span: CodeSpan,
        cur_type: CursorType,
        return_type: CursorType,
    },
    Variable(String, CodeSpan, CursorType),
    Parameter(String, CodeSpan, CursorType),
    Typedef(String, CodeSpan, AccessSpecifierType),
    Method {
        spelling: String,
        code_span: CodeSpan,
        access_specifier: AccessSpecifierType,
        cur_type: CursorType,
        virtuality: Virtuality,
        return_type: CursorType,
    },
    Namespace(String, CodeSpan),
    LinkageSpec(String, CodeSpan),
    Constructor(String, CodeSpan, ConstructorType, AccessSpecifierType),
    Destructor(String, CodeSpan, Virtuality, AccessSpecifierType),
    ConversionFunction(String, CodeSpan, AccessSpecifierType),
    TemplateTypeParameter(String, CodeSpan),
    TemplateNonTypeParameter(String, CodeSpan),
    TemplateTemplateParameter(String, CodeSpan),
    FunctionTemplate(String, CodeSpan),
    ClassTemplate(String, CodeSpan),
    ClassTemplatePartial(String, CodeSpan),
    NamespaceAlias(String, CodeSpan),
    UsingDirective(String, CodeSpan),
    TypeAlias(String, CodeSpan, AccessSpecifierType),
    AccessSpecifier(CodeSpan, AccessSpecifierType),
    TypeReference(String, CodeSpan),
    BaseSpecifier(String, CodeSpan),
    TemplateReference(String, CodeSpan),
    NamespaceReference(String, CodeSpan),
    MemberReference(String, CodeSpan),
    LabelReference(String, CodeSpan),
    OverloadedDeclarationReference(String, CodeSpan),
    VariableReference(String),
    CodeSpan,
    UnexposedExpression(String, CodeSpan),
    DeclarationReferenceExpression(String, CodeSpan), // TODO what is this?
    MemberReferenceExpression(String, CodeSpan),
    CallExpression(String, CodeSpan),  // TODO what is this?
    BlockExpression(String, CodeSpan), // TODO what is this?
    IntegerLiteral(String, CodeSpan),
    FloatLiteral(CodeSpan),
    ImaginaryLiteral(String, CodeSpan), // TODO what is this?
    StringLiteral(String, CodeSpan),
    CharacterLiteral(String, CodeSpan),
    UnaryOperator(String, CodeSpan),
    ArraySubscription(String, CodeSpan),
    BinaryOperator(CodeSpan),
    CompoundAssignOperator(String, CodeSpan),
    ConditionalOperator(String, CodeSpan),
    CStyleCast(String, CodeSpan),
    CompoundLiteralExpression(String, CodeSpan), // TODO what is this?
    InitializerListExpression(String, CodeSpan),
    CompoundStatement(CodeSpan),
    ReturnStatement(CodeSpan),
    NotSupported(String, CodeSpan, i32),
}

fn get_cursor_type(cursor: CXCursor) -> i32 {
    unsafe { clang_getCursorType(cursor).kind }
}

fn get_cursor_return_type(cursor: CXCursor) -> i32 {
    unsafe { clang_getCursorResultType(cursor).kind }
}

fn get_cursor_canonical_type(cursor: CXCursor) -> i32 {
    unsafe { get_canonical_type(clang_getCursorType(cursor)) }
}

fn get_cursor_canonical_return_type(cursor: CXCursor) -> i32 {
    unsafe { get_canonical_type(clang_getCursorResultType(cursor)) }
}

fn get_canonical_type(cur_type: CXType) -> i32 {
    unsafe { clang_getCanonicalType(cur_type).kind }
}

fn get_cursor_virtuality(cursor: CXCursor) -> Virtuality {
    unsafe {
        if clang_CXXMethod_isPureVirtual(cursor) == 1 {
            Virtuality::PureVirtual
        } else if clang_CXXMethod_isVirtual(cursor) == 1 {
            Virtuality::Virtual
        } else if clang_CXXMethod_isStatic(cursor) == 1 {
            Virtuality::Static
        } else {
            Virtuality::NonVirtual
        }
    }
}

fn get_constructor_type(cursor: CXCursor) -> ConstructorType {
    unsafe {
        if clang_CXXConstructor_isCopyConstructor(cursor) == 1 {
            ConstructorType::Copy
        } else if clang_CXXConstructor_isMoveConstructor(cursor) == 1 {
            ConstructorType::Move
        } else if clang_CXXConstructor_isDefaultConstructor(cursor) == 1 {
            ConstructorType::Default
        } else if clang_CXXConstructor_isConvertingConstructor(cursor) == 1 {
            ConstructorType::Converting
        } else {
            ConstructorType::None
        }
    }
}

fn get_cursor_extent(cursor: CXCursor) -> CodeSpan {
    unsafe {
        let extent = clang_getCursorExtent(cursor);
        let begin = clang_getRangeStart(extent);
        let end = clang_getRangeEnd(extent);
        let mut begin_line: u32 = 0;
        let mut begin_col: u32 = 0;
        let mut begin_file: CXFile = ptr::null_mut();
        let mut end_line: u32 = 0;
        let mut end_col: u32 = 0;
        let mut end_file: CXFile = ptr::null_mut();
        clang_getSpellingLocation(
            begin,
            &mut begin_file,
            &mut begin_line,
            &mut begin_col,
            ptr::null_mut(),
        );
        clang_getSpellingLocation(
            end,
            &mut end_file,
            &mut end_line,
            &mut end_col,
            ptr::null_mut(),
        );
        CodeSpan {
            start_pos: Position {
                file_name: convert_into_owned(clang_getFileName(begin_file)),
                line: begin_line,
                col: begin_col,
            },
            end_pos: Position {
                file_name: convert_into_owned(clang_getFileName(end_file)),
                line: end_line,
                col: end_col,
            },
        }
    }
}

impl From<i32> for CursorType {
    fn from(cursor_type: i32) -> Self {
        match cursor_type {
            clang_sys::CXType_Unexposed => CursorType::Unexposed,
            clang_sys::CXType_Void => CursorType::Void,
            clang_sys::CXType_Bool => CursorType::Bool,
            clang_sys::CXType_Char_U => CursorType::CharU,
            clang_sys::CXType_UChar => CursorType::UnsignedChar,
            clang_sys::CXType_Char16 => CursorType::Char16,
            clang_sys::CXType_Char32 => CursorType::Char32,
            clang_sys::CXType_UShort => CursorType::UnsignedShort,
            clang_sys::CXType_UInt => CursorType::UnsignedInt,
            clang_sys::CXType_ULong => CursorType::UnsignedLong,
            clang_sys::CXType_ULongLong => CursorType::UnsignedLongLong,
            clang_sys::CXType_UInt128 => CursorType::UnsignedInt128,
            clang_sys::CXType_Char_S => CursorType::CharS,
            clang_sys::CXType_SChar => CursorType::SignedChar,
            clang_sys::CXType_WChar => CursorType::WideChar,
            clang_sys::CXType_Short => CursorType::Short,
            clang_sys::CXType_Int => CursorType::Int,
            clang_sys::CXType_Long => CursorType::Long,
            clang_sys::CXType_LongLong => CursorType::LongLong,
            clang_sys::CXType_Int128 => CursorType::Int128,
            clang_sys::CXType_Float => CursorType::Float,
            clang_sys::CXType_Double => CursorType::Double,
            clang_sys::CXType_LongDouble => CursorType::LongDouble,
            clang_sys::CXType_NullPtr => CursorType::Nullptr,
            clang_sys::CXType_Overload => CursorType::Overload,
            clang_sys::CXType_Dependent => CursorType::Dependent,
            clang_sys::CXType_Float128 => CursorType::Float128,
            clang_sys::CXType_Half => CursorType::Half,
            clang_sys::CXType_Float16 => CursorType::Float16,
            clang_sys::CXType_ShortAccum => CursorType::ShortAccum,
            clang_sys::CXType_Accum => CursorType::Accum,
            clang_sys::CXType_LongAccum => CursorType::LongAccum,
            clang_sys::CXType_UShortAccum => CursorType::UnsignedShortAccum,
            clang_sys::CXType_UAccum => CursorType::UnsignedAccum,
            clang_sys::CXType_ULongAccum => CursorType::UnsignedLongAccum,
            clang_sys::CXType_Complex => CursorType::Complex,
            clang_sys::CXType_Pointer => CursorType::Pointer,
            clang_sys::CXType_BlockPointer => CursorType::BlockPointer,
            clang_sys::CXType_LValueReference => CursorType::LValueReference,
            clang_sys::CXType_RValueReference => CursorType::RValueReference,
            clang_sys::CXType_Record => CursorType::Record,
            clang_sys::CXType_Enum => CursorType::Enum,
            clang_sys::CXType_Typedef => CursorType::Typedef,
            clang_sys::CXType_FunctionNoProto => CursorType::FunctionNoProto,
            clang_sys::CXType_FunctionProto => CursorType::FunctionProto,
            clang_sys::CXType_ConstantArray => CursorType::ConstantArray,
            clang_sys::CXType_Vector => CursorType::Vector,
            clang_sys::CXType_IncompleteArray => CursorType::IncompleteArray,
            clang_sys::CXType_VariableArray => CursorType::VariableArray,
            clang_sys::CXType_DependentSizedArray => CursorType::DependentSizedArray,
            clang_sys::CXType_MemberPointer => CursorType::MemberPointer,
            clang_sys::CXType_Auto => CursorType::Auto,
            clang_sys::CXType_Elaborated => CursorType::Elaborated,
            clang_sys::CXType_Pipe => CursorType::Pipe,
            clang_sys::CXType_Attributed => CursorType::Attributed,
            _ => CursorType::NotSupported(cursor_type),
        }
    }
}

fn get_access_specifier(cursor: CXCursor) -> i32 {
    unsafe { clang_getCXXAccessSpecifier(cursor) }
}

impl From<i32> for AccessSpecifierType {
    fn from(access_specifier_type: i32) -> Self {
        match access_specifier_type {
            clang_sys::CX_CXXPrivate => AccessSpecifierType::Private,
            clang_sys::CX_CXXProtected => AccessSpecifierType::Protected,
            clang_sys::CX_CXXPublic => AccessSpecifierType::Public,
            _ => AccessSpecifierType::Invalid,
        }
    }
}

fn convert_into_owned(clang_string: CXString) -> String {
    unsafe {
        let string = clang_getCString(clang_string);
        let string = CStr::from_ptr(string).to_string_lossy().into_owned();
        clang_disposeString(clang_string);
        string
    }
}

impl From<CXCursor> for CursorKind {
    fn from(cursor: CXCursor) -> Self {
        unsafe {
            let cursor_spelling = clang_getCursorSpelling(cursor);
            let cursor_kind = clang_getCursorKind(cursor);

            let spelling = convert_into_owned(cursor_spelling);
            let cursor_kind = match cursor_kind {
                clang_sys::CXCursor_UnexposedDecl => CursorKind::Unexposed(spelling),
                clang_sys::CXCursor_StructDecl => CursorKind::Struct(
                    spelling,
                    get_cursor_extent(cursor),
                    get_access_specifier(cursor).into(),
                ),
                clang_sys::CXCursor_UnionDecl => CursorKind::Union(
                    spelling,
                    get_cursor_extent(cursor),
                    get_access_specifier(cursor).into(),
                ),
                clang_sys::CXCursor_ClassDecl => CursorKind::Class(
                    spelling,
                    get_cursor_extent(cursor),
                    get_access_specifier(cursor).into(),
                ),
                clang_sys::CXCursor_FieldDecl => CursorKind::Field(
                    spelling,
                    get_cursor_extent(cursor),
                    get_access_specifier(cursor).into(),
                    get_cursor_type(cursor).into(),
                ),
                clang_sys::CXCursor_EnumDecl => CursorKind::Enum(
                    spelling,
                    get_cursor_extent(cursor),
                    get_access_specifier(cursor).into(),
                ),
                clang_sys::CXCursor_EnumConstantDecl => {
                    CursorKind::EnumConstant(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_FunctionDecl => CursorKind::Function {
                    spelling,
                    code_span: get_cursor_extent(cursor),
                    cur_type: get_cursor_type(cursor).into(),
                    return_type: get_cursor_return_type(cursor).into(),
                },
                clang_sys::CXCursor_VarDecl => CursorKind::Variable(
                    spelling,
                    get_cursor_extent(cursor),
                    get_cursor_type(cursor).into(),
                ),
                clang_sys::CXCursor_ParmDecl => CursorKind::Parameter(
                    spelling,
                    get_cursor_extent(cursor),
                    get_cursor_type(cursor).into(),
                ),
                clang_sys::CXCursor_TypedefDecl => CursorKind::Typedef(
                    spelling,
                    get_cursor_extent(cursor),
                    get_access_specifier(cursor).into(),
                ),
                clang_sys::CXCursor_CXXMethod => CursorKind::Method {
                    spelling,
                    code_span: get_cursor_extent(cursor),
                    access_specifier: get_access_specifier(cursor).into(),
                    cur_type: get_cursor_type(cursor).into(),
                    virtuality: get_cursor_virtuality(cursor),
                    return_type: get_cursor_return_type(cursor).into(),
                },
                clang_sys::CXCursor_Namespace => {
                    CursorKind::Namespace(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_LinkageSpec => {
                    CursorKind::LinkageSpec(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_Constructor => CursorKind::Constructor(
                    spelling,
                    get_cursor_extent(cursor),
                    get_constructor_type(cursor),
                    get_access_specifier(cursor).into(),
                ),
                clang_sys::CXCursor_Destructor => CursorKind::Destructor(
                    spelling,
                    get_cursor_extent(cursor),
                    get_cursor_virtuality(cursor),
                    get_access_specifier(cursor).into(),
                ),
                clang_sys::CXCursor_ConversionFunction => CursorKind::ConversionFunction(
                    spelling,
                    get_cursor_extent(cursor),
                    get_access_specifier(cursor).into(),
                ),
                clang_sys::CXCursor_TemplateTypeParameter => {
                    CursorKind::TemplateTypeParameter(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_NonTypeTemplateParameter => {
                    CursorKind::TemplateNonTypeParameter(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_TemplateTemplateParameter => {
                    CursorKind::TemplateTemplateParameter(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_FunctionTemplate => {
                    CursorKind::FunctionTemplate(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_ClassTemplate => {
                    CursorKind::ClassTemplate(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_ClassTemplatePartialSpecialization => {
                    CursorKind::ClassTemplatePartial(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_NamespaceAlias => {
                    CursorKind::NamespaceAlias(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_UsingDirective => {
                    CursorKind::UsingDirective(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_TypeAliasDecl => CursorKind::TypeAlias(
                    spelling,
                    get_cursor_extent(cursor),
                    get_access_specifier(cursor).into(),
                ),
                clang_sys::CXCursor_CXXAccessSpecifier => CursorKind::AccessSpecifier(
                    get_cursor_extent(cursor),
                    get_access_specifier(cursor).into(),
                ),
                clang_sys::CXCursor_TypeRef => {
                    CursorKind::TypeReference(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_CXXBaseSpecifier => {
                    CursorKind::BaseSpecifier(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_BinaryOperator => {
                    CursorKind::BinaryOperator(get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_DeclRefExpr => {
                    CursorKind::DeclarationReferenceExpression(spelling, get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_FloatingLiteral => {
                    CursorKind::FloatLiteral(get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_CompoundStmt => {
                    CursorKind::CompoundStatement(get_cursor_extent(cursor))
                }
                clang_sys::CXCursor_ReturnStmt => {
                    CursorKind::ReturnStatement(get_cursor_extent(cursor))
                }
                _ => CursorKind::NotSupported(spelling, get_cursor_extent(cursor), cursor_kind),
            };
            cursor_kind
        }
    }
}
