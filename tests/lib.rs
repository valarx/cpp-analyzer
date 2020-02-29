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
                    code_span: CodeSpan {
                        start_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 4,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 4,
                            col: 22
                        }
                    },
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::Int
                }
            );
            assert_eq!(
                cursors[1],
                CursorKind::Parameter(
                    "a".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 4,
                            col: 9
                        },
                        end_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 4,
                            col: 14
                        }
                    },
                    CursorType::Int
                )
            );
            assert_eq!(
                cursors[2],
                CursorKind::Parameter(
                    "b".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 4,
                            col: 16
                        },
                        end_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 4,
                            col: 21
                        }
                    },
                    CursorType::Int
                )
            );
            assert_eq!(
                cursors[3],
                CursorKind::Function {
                    spelling: "function_with_param".to_owned(),
                    code_span: CodeSpan {
                        start_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 6,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 9,
                            col: 2
                        }
                    },
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::LValueReference
                }
            );
            assert_eq!(
                cursors[4],
                CursorKind::Parameter(
                    "k".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 6,
                            col: 28
                        },
                        end_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 6,
                            col: 36
                        }
                    },
                    CursorType::LValueReference
                )
            );
            assert_eq!(
                cursors[5],
                CursorKind::CompoundStatement(CodeSpan {
                    start_pos: Position {
                        file_name: "tests/header.h".to_owned(),
                        line: 6,
                        col: 38
                    },
                    end_pos: Position {
                        file_name: "tests/header.h".to_owned(),
                        line: 9,
                        col: 2
                    }
                },)
            );
            assert_eq!(
                cursors[6],
                CursorKind::BinaryOperator(CodeSpan {
                    start_pos: Position {
                        file_name: "tests/header.h".to_owned(),
                        line: 7,
                        col: 3
                    },
                    end_pos: Position {
                        file_name: "tests/header.h".to_owned(),
                        line: 7,
                        col: 10
                    }
                },)
            );
            assert_eq!(
                cursors[7],
                CursorKind::DeclarationReferenceExpression(
                    "k".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 7,
                            col: 3
                        },
                        end_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 7,
                            col: 4
                        }
                    },
                )
            );
            assert_eq!(
                cursors[8],
                CursorKind::FloatLiteral(CodeSpan {
                    start_pos: Position {
                        file_name: "tests/header.h".to_owned(),
                        line: 7,
                        col: 7
                    },
                    end_pos: Position {
                        file_name: "tests/header.h".to_owned(),
                        line: 7,
                        col: 10
                    }
                })
            );
            assert_eq!(
                cursors[9],
                CursorKind::ReturnStatement(CodeSpan {
                    start_pos: Position {
                        file_name: "tests/header.h".to_owned(),
                        line: 8,
                        col: 3
                    },
                    end_pos: Position {
                        file_name: "tests/header.h".to_owned(),
                        line: 8,
                        col: 11
                    }
                })
            );

            assert_eq!(
                cursors[10],
                CursorKind::DeclarationReferenceExpression(
                    "k".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 8,
                            col: 10
                        },
                        end_pos: Position {
                            file_name: "tests/header.h".to_owned(),
                            line: 8,
                            col: 11
                        }
                    },
                )
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
            assert_eq!(
                cursors[0],
                CursorKind::Namespace(
                    "my_namespace".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 4,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 20,
                            col: 2
                        }
                    },
                )
            );
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
                CursorKind::Struct(
                    "PrivateStruct".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 6,
                            col: 3
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 6,
                            col: 26
                        }
                    },
                    AccessSpecifierType::Private
                )
            );
            assert_eq!(
                cursors[3],
                CursorKind::Field(
                    "field".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 7,
                            col: 3
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 7,
                            col: 12
                        }
                    },
                    AccessSpecifierType::Private,
                    CursorType::Int
                )
            );
            assert_eq!(
                cursors[4],
                CursorKind::AccessSpecifier(
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 9,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 9,
                            col: 8
                        }
                    },
                    AccessSpecifierType::Public
                )
            );
            assert_eq!(
                cursors[5],
                CursorKind::Field(
                    "field1".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 10,
                            col: 3
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 10,
                            col: 15
                        }
                    },
                    AccessSpecifierType::Public,
                    CursorType::Float
                )
            );
            assert_eq!(
                cursors[6],
                CursorKind::Constructor(
                    "MyTestClass".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 11,
                            col: 3
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 11,
                            col: 16
                        }
                    },
                    ConstructorType::Default,
                    AccessSpecifierType::Public
                )
            );
            assert_eq!(
                cursors[7],
                CursorKind::Constructor(
                    "MyTestClass".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 12,
                            col: 3
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 12,
                            col: 35
                        }
                    },
                    ConstructorType::Copy,
                    AccessSpecifierType::Public
                )
            );
            assert_eq!(
                cursors[8],
                CursorKind::Parameter(
                    "".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 12,
                            col: 15
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 12,
                            col: 34
                        }
                    },
                    CursorType::LValueReference
                )
            );
            assert_eq!(
                cursors[9],
                CursorKind::TypeReference(
                    "class my_namespace::MyTestClass".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 12,
                            col: 21
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 12,
                            col: 32
                        }
                    },
                )
            );
            assert_eq!(
                cursors[10],
                CursorKind::Method {
                    spelling: "operator=".to_owned(),
                    code_span: CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 13,
                            col: 3
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 13,
                            col: 46
                        }
                    },
                    access_specifier: AccessSpecifierType::Public,
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::LValueReference
                }
            );
            assert_eq!(
                cursors[11],
                CursorKind::TypeReference(
                    "class my_namespace::MyTestClass".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 13,
                            col: 3
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 13,
                            col: 14
                        }
                    },
                )
            );
            assert_eq!(
                cursors[12],
                CursorKind::Parameter(
                    "".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 13,
                            col: 26
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 13,
                            col: 45
                        }
                    },
                    CursorType::LValueReference
                )
            );
            assert_eq!(
                cursors[13],
                CursorKind::TypeReference(
                    "class my_namespace::MyTestClass".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 13,
                            col: 32
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 13,
                            col: 43
                        }
                    },
                )
            );
            assert_eq!(
                cursors[14],
                CursorKind::AccessSpecifier(
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 15,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 15,
                            col: 11
                        }
                    },
                    AccessSpecifierType::Protected
                )
            );
            assert_eq!(
                cursors[15],
                CursorKind::Field(
                    "field3".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 16,
                            col: 3
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 16,
                            col: 14
                        }
                    },
                    AccessSpecifierType::Protected,
                    CursorType::Bool
                )
            );
            assert_eq!(
                cursors[16],
                CursorKind::Method {
                    spelling: "test_method".to_owned(),
                    code_span: CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 17,
                            col: 3
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 17,
                            col: 21
                        }
                    },
                    access_specifier: AccessSpecifierType::Protected,
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::Void
                }
            );
            assert_eq!(
                cursors[17],
                CursorKind::Constructor(
                    "MyTestClass".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 18,
                            col: 3
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 18,
                            col: 30
                        }
                    },
                    ConstructorType::Move,
                    AccessSpecifierType::Protected
                )
            );

            assert_eq!(
                cursors[18],
                CursorKind::Parameter(
                    "".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 18,
                            col: 15
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 18,
                            col: 29
                        }
                    },
                    CursorType::RValueReference
                )
            );
            assert_eq!(
                cursors[19],
                CursorKind::TypeReference(
                    "class my_namespace::MyTestClass".to_owned(),
                    CodeSpan {
                        start_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 18,
                            col: 15
                        },
                        end_pos: Position {
                            file_name: "tests/class.h".to_owned(),
                            line: 18,
                            col: 26
                        }
                    },
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
