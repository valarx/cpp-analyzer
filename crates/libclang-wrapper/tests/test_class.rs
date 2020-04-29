use libclang_wrapper;
use libclang_wrapper::source::{
    AccessSpecifierType, CodeSpan, ConstructorType, CursorKind, CursorType, DeclarationFromPHCMode,
    DiagnosticsMode, Entry, Position, Source, TUOptionsBuilder, Virtuality,
};

#[test]
fn parse_class_in_namespace() {
    let source = Source::from_file(
        "tests/class.h".to_owned(),
        DeclarationFromPHCMode::Exclude,
        DiagnosticsMode::Enabled,
        vec!["-x".to_owned(), "c++".to_owned()],
        TUOptionsBuilder::new(),
    )
    .unwrap();
    let translation_units: Result<Vec<_>, _> = source.translation_units.into_iter().collect();
    let translation_unit = translation_units.unwrap().into_iter().nth(0).unwrap();
    let ast = translation_unit.ast();
    assert_eq!(
        ast,
        &Entry {
            current_kind: CursorKind::Root,
            children: vec![Entry {
                current_kind: CursorKind::Namespace(
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
                    }
                ),
                children: vec![Entry {
                    current_kind: CursorKind::Class(
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
                    ),
                    children: vec![
                        Entry {
                            current_kind: CursorKind::Struct(
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
                            ),
                            children: vec![]
                        },
                        Entry {
                            current_kind: CursorKind::Field(
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
                            ),
                            children: vec![]
                        },
                        Entry {
                            current_kind: CursorKind::AccessSpecifier(
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
                            ),
                            children: vec![]
                        },
                        Entry {
                            current_kind: CursorKind::Field(
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
                            ),
                            children: vec![]
                        },
                        Entry {
                            current_kind: CursorKind::Constructor(
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
                            ),
                            children: vec![]
                        },
                        Entry {
                            current_kind: CursorKind::Constructor(
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
                            ),
                            children: vec![Entry {
                                current_kind: CursorKind::Parameter(
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
                                ),
                                children: vec![Entry {
                                    current_kind: CursorKind::TypeReference(
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
                                        }
                                    ),
                                    children: vec![]
                                }]
                            }]
                        },
                        Entry {
                            current_kind: CursorKind::Method {
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
                                        col: 58
                                    }
                                },
                                access_specifier: AccessSpecifierType::Public,
                                cur_type: CursorType::FunctionProto,
                                virtuality: Virtuality::PureVirtual,
                                return_type: CursorType::LValueReference,
                                canonical_return_type: CursorType::LValueReference
                            },
                            children: vec![
                                Entry {
                                    current_kind: CursorKind::TypeReference(
                                        "class my_namespace::MyTestClass".to_owned(),
                                        CodeSpan {
                                            start_pos: Position {
                                                file_name: "tests/class.h".to_owned(),
                                                line: 13,
                                                col: 11
                                            },
                                            end_pos: Position {
                                                file_name: "tests/class.h".to_owned(),
                                                line: 13,
                                                col: 22
                                            }
                                        }
                                    ),
                                    children: vec![]
                                },
                                Entry {
                                    current_kind: CursorKind::Parameter(
                                        "".to_owned(),
                                        CodeSpan {
                                            start_pos: Position {
                                                file_name: "tests/class.h".to_owned(),
                                                line: 13,
                                                col: 34
                                            },
                                            end_pos: Position {
                                                file_name: "tests/class.h".to_owned(),
                                                line: 13,
                                                col: 53
                                            }
                                        },
                                        CursorType::LValueReference
                                    ),
                                    children: vec![Entry {
                                        current_kind: CursorKind::TypeReference(
                                            "class my_namespace::MyTestClass".to_owned(),
                                            CodeSpan {
                                                start_pos: Position {
                                                    file_name: "tests/class.h".to_owned(),
                                                    line: 13,
                                                    col: 40
                                                },
                                                end_pos: Position {
                                                    file_name: "tests/class.h".to_owned(),
                                                    line: 13,
                                                    col: 51
                                                }
                                            }
                                        ),
                                        children: vec![]
                                    }]
                                }
                            ]
                        },
                        Entry {
                            current_kind: CursorKind::AccessSpecifier(
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
                            ),
                            children: vec![]
                        },
                        Entry {
                            current_kind: CursorKind::Field(
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
                            ),
                            children: vec![]
                        },
                        Entry {
                            current_kind: CursorKind::Method {
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
                                virtuality: Virtuality::NonVirtual,
                                return_type: CursorType::Void,
                                canonical_return_type: CursorType::Void
                            },
                            children: vec![]
                        },
                        Entry {
                            current_kind: CursorKind::Constructor(
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
                            ),
                            children: vec![Entry {
                                current_kind: CursorKind::Parameter(
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
                                ),
                                children: vec![Entry {
                                    current_kind: CursorKind::TypeReference(
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
                                        }
                                    ),
                                    children: vec![]
                                }]
                            }]
                        }
                    ]
                }]
            }]
        }
    );
}
