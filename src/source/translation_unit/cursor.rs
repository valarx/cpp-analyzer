use clang_sys::*;
use std::ffi::CStr;

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
pub enum CursorKind {
    Unexposed(String),
    Struct(String, AccessSpecifierType),
    Union(String, AccessSpecifierType),
    Class(String, AccessSpecifierType),
    Field(String, AccessSpecifierType, CursorType),
    Enum(String, AccessSpecifierType),
    EnumConstant(String),
    Function {
        spelling: String,
        cur_type: CursorType,
    },
    Variable(String, CursorType),
    Parameter(String, CursorType),
    Typedef(String, AccessSpecifierType),
    Method {
        spelling: String,
        access_specifier: AccessSpecifierType,
        cur_type: CursorType,
    },
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
    TemplateReference(String),
    NamespaceReference(String),
    MemberReference(String),
    LabelReference(String),
    OverloadedDeclarationReference(String),
    VariableReference(String),
    UnexposedExpression(String),
    DeclarationReferenceExpression(String), // TODO what is this?
    MemberReferenceExpression(String),
    CallExpression(String),  // TODO what is this?
    BlockExpression(String), // TODO what is this?
    IntegerLiteral(String),
    FloatLiteral(String),
    ImaginaryLiteral(String), // TODO what is this?
    StringLiteral(String),
    CharacterLiteral(String),
    UnaryOperator(String),
    ArraySubscription(String),
    BinaryOperator(String),
    CompoundAssignOperator(String),
    ConditionalOperator(String),
    CStyleCast(String),
    CompoundLiteralExpression(String), // TODO what is this?
    InitializerListExpression(String),
    NotSupported(String, i32),
}

impl From<CXCursor> for CursorType {
    fn from(cursor: CXCursor) -> Self {
        unsafe {
            let cursor_type = clang_getCursorType(cursor).kind;
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
}

impl From<CXCursor> for AccessSpecifierType {
    fn from(cursor: CXCursor) -> Self {
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
}

impl From<CXCursor> for CursorKind {
    fn from(cursor: CXCursor) -> Self {
        unsafe {
            let cursor_spelling = clang_getCursorSpelling(cursor);
            let cursor_kind = clang_getCursorKind(cursor);

            let spelling = clang_getCString(cursor_spelling);
            let spelling = CStr::from_ptr(spelling).to_string_lossy().into_owned();
            let cursor_kind = match cursor_kind {
                clang_sys::CXCursor_UnexposedDecl => CursorKind::Unexposed(spelling),
                clang_sys::CXCursor_StructDecl => CursorKind::Struct(spelling, cursor.into()),
                clang_sys::CXCursor_UnionDecl => CursorKind::Union(spelling, cursor.into()),
                clang_sys::CXCursor_ClassDecl => CursorKind::Class(spelling, cursor.into()),
                clang_sys::CXCursor_FieldDecl => {
                    CursorKind::Field(spelling, cursor.into(), cursor.into())
                }
                clang_sys::CXCursor_EnumDecl => CursorKind::Enum(spelling, cursor.into()),
                clang_sys::CXCursor_EnumConstantDecl => CursorKind::EnumConstant(spelling),
                clang_sys::CXCursor_FunctionDecl => CursorKind::Function {
                    spelling,
                    cur_type: cursor.into(),
                },
                clang_sys::CXCursor_VarDecl => CursorKind::Variable(spelling, cursor.into()),
                clang_sys::CXCursor_ParmDecl => CursorKind::Parameter(spelling, cursor.into()),
                clang_sys::CXCursor_TypedefDecl => CursorKind::Typedef(spelling, cursor.into()),
                clang_sys::CXCursor_CXXMethod => CursorKind::Method {
                    spelling,
                    access_specifier: cursor.into(),
                    cur_type: cursor.into(),
                },
                clang_sys::CXCursor_Namespace => CursorKind::Namespace(spelling),
                clang_sys::CXCursor_LinkageSpec => CursorKind::LinkageSpec(spelling),
                clang_sys::CXCursor_Constructor => CursorKind::Constructor(spelling, cursor.into()),
                clang_sys::CXCursor_Destructor => CursorKind::Destructor(spelling, cursor.into()),
                clang_sys::CXCursor_ConversionFunction => {
                    CursorKind::ConversionFunction(spelling, cursor.into())
                }
                clang_sys::CXCursor_TemplateTypeParameter => {
                    CursorKind::TemplateTypeParameter(spelling)
                }
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
                clang_sys::CXCursor_TypeAliasDecl => CursorKind::TypeAlias(spelling, cursor.into()),
                clang_sys::CXCursor_CXXAccessSpecifier => {
                    CursorKind::AccessSpecifier(cursor.into())
                }
                clang_sys::CXCursor_TypeRef => CursorKind::TypeReference(spelling),
                clang_sys::CXCursor_CXXBaseSpecifier => CursorKind::BaseSpecifier(spelling),
                _ => CursorKind::NotSupported(spelling, cursor_kind),
            };
            clang_disposeString(cursor_spelling);
            cursor_kind
        }
    }
}