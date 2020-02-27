use clang_sys::*;
use libclang_wrapper;
use libclang_wrapper::source;

use libclang_wrapper::source::{
    AccessSpecifierType, CodeSpan, ConstructorType, CursorKind, CursorType, Position,
};

#[test]
fn parse_single_function() {
    let source = source::Source::new(
        "tests/header.h".to_owned(),
        source::DeclarationFromPHCMode::Exclude,
        source::DiagnosticsMode::Enabled,
        vec!["-x".to_owned(), "c++".to_owned()],
        source::TUOptionsBuilder::new(),
    );
    match source {
        Ok(source) => {
            let cursors = source.translation_units[0].get_cursors();
            assert_eq!(
                cursors[0],
                CursorKind::Function {
                    spelling: "add".to_owned(),
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::Int
                }
            );
            assert_eq!(
                cursors[1],
                CursorKind::Parameter("a".to_owned(), CursorType::Int)
            );
            assert_eq!(
                cursors[2],
                CursorKind::Parameter("b".to_owned(), CursorType::Int)
            );
            assert_eq!(
                cursors[3],
                CursorKind::Function {
                    spelling: "function_with_param".to_owned(),
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::LValueReference
                }
            );
            assert_eq!(
                cursors[4],
                CursorKind::Parameter("k".to_owned(), CursorType::LValueReference)
            );
            assert_eq!(cursors[5], CursorKind::CompoundStatement);
            assert_eq!(cursors[6], CursorKind::BinaryOperator);
            assert_eq!(
                cursors[7],
                CursorKind::DeclarationReferenceExpression("k".to_owned())
            );
            assert_eq!(cursors[8], CursorKind::FloatLiteral);
            assert_eq!(cursors[9], CursorKind::ReturnStatement);

            assert_eq!(
                cursors[10],
                CursorKind::DeclarationReferenceExpression("k".to_owned())
            );
        }
        Err(error) => panic!("{:?}", error),
    };
}

#[test]
fn parse_class_in_namespace() {
    let source = source::Source::new(
        "tests/class.h".to_owned(),
        source::DeclarationFromPHCMode::Exclude,
        source::DiagnosticsMode::Enabled,
        vec!["-x".to_owned(), "c++".to_owned()],
        source::TUOptionsBuilder::new(),
    );
    match source {
        Ok(source) => {
            let cursors = source.translation_units[0].get_cursors();
            assert_eq!(cursors[0], CursorKind::Namespace("my_namespace".to_owned()));
            assert_eq!(
                cursors[1],
                CursorKind::Class(
                    "MyTestClass".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 5,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 19,
                            col: 2
                        }
                    },
                    AccessSpecifierType::Invalid
                )
            );
            assert_eq!(
                cursors[2],
                CursorKind::Struct("PrivateStruct".to_owned(), AccessSpecifierType::Private)
            );
            assert_eq!(
                cursors[3],
                CursorKind::Field(
                    "field".to_owned(),
                    AccessSpecifierType::Private,
                    CursorType::Int
                )
            );
            assert_eq!(
                cursors[4],
                CursorKind::AccessSpecifier(AccessSpecifierType::Public)
            );
            assert_eq!(
                cursors[5],
                CursorKind::Field(
                    "field1".to_owned(),
                    AccessSpecifierType::Public,
                    CursorType::Float
                )
            );
            assert_eq!(
                cursors[6],
                CursorKind::Constructor(
                    "MyTestClass".to_owned(),
                    ConstructorType::Default,
                    AccessSpecifierType::Public
                )
            );
            assert_eq!(
                cursors[7],
                CursorKind::Constructor(
                    "MyTestClass".to_owned(),
                    ConstructorType::Copy,
                    AccessSpecifierType::Public
                )
            );
            assert_eq!(
                cursors[8],
                CursorKind::Parameter("".to_owned(), CursorType::LValueReference)
            );
            assert_eq!(
                cursors[9],
                CursorKind::TypeReference("class my_namespace::MyTestClass".to_owned())
            );
            assert_eq!(
                cursors[10],
                CursorKind::Method {
                    spelling: "operator=".to_owned(),
                    access_specifier: AccessSpecifierType::Public,
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::LValueReference
                }
            );
            assert_eq!(
                cursors[11],
                CursorKind::TypeReference("class my_namespace::MyTestClass".to_owned())
            );
            assert_eq!(
                cursors[12],
                CursorKind::Parameter("".to_owned(), CursorType::LValueReference)
            );
            assert_eq!(
                cursors[13],
                CursorKind::TypeReference("class my_namespace::MyTestClass".to_owned())
            );
            assert_eq!(
                cursors[14],
                CursorKind::AccessSpecifier(AccessSpecifierType::Protected)
            );
            assert_eq!(
                cursors[15],
                CursorKind::Field(
                    "field3".to_owned(),
                    AccessSpecifierType::Protected,
                    CursorType::Bool
                )
            );
            assert_eq!(
                cursors[16],
                CursorKind::Method {
                    spelling: "test_method".to_owned(),
                    access_specifier: AccessSpecifierType::Protected,
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::Void
                }
            );
            assert_eq!(
                cursors[17],
                CursorKind::Constructor(
                    "MyTestClass".to_owned(),
                    ConstructorType::Move,
                    AccessSpecifierType::Protected
                )
            );
        }
        Err(error) => panic!("{:?}", error),
    };
}

#[test]
fn test_support() {
    let clang = support::Clang::find(None, &[]).unwrap();
    println!("{:?}", clang);
}
