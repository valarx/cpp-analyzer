use libclang_wrapper;
use libclang_wrapper::source::{
    CodeSpan, CursorKind, CursorType, DeclarationFromPHCMode, DiagnosticsMode, Entry, Position,
    Source, TUOptionsBuilder,
};

#[test]
fn test_while_loop() {
    let source = Source::from_file(
        "tests/while.cpp".to_owned(),
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
                current_kind: CursorKind::Function {
                    spelling: "f".to_owned(),
                    display_name: "f()".to_owned(),
                    code_span: CodeSpan {
                        start_pos: Position {
                            file_name: "tests/while.cpp".to_owned(),
                            line: 1,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/while.cpp".to_owned(),
                            line: 6,
                            col: 2
                        }
                    },
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::Void,
                    canonical_return_type: CursorType::Void
                },
                children: vec![Entry {
                    current_kind: CursorKind::CompoundStatement(CodeSpan {
                        start_pos: Position {
                            file_name: "tests/while.cpp".to_owned(),
                            line: 1,
                            col: 10
                        },
                        end_pos: Position {
                            file_name: "tests/while.cpp".to_owned(),
                            line: 6,
                            col: 2
                        }
                    }),
                    children: vec![
                        Entry {
                            current_kind: CursorKind::DeclarationStatement(CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/while.cpp".to_owned(),
                                    line: 2,
                                    col: 3
                                },
                                end_pos: Position {
                                    file_name: "tests/while.cpp".to_owned(),
                                    line: 2,
                                    col: 14
                                }
                            }),
                            children: vec![Entry {
                                current_kind: CursorKind::Variable(
                                    "i".to_owned(),
                                    CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/while.cpp".to_owned(),
                                            line: 2,
                                            col: 3
                                        },
                                        end_pos: Position {
                                            file_name: "tests/while.cpp".to_owned(),
                                            line: 2,
                                            col: 13
                                        }
                                    },
                                    CursorType::Int
                                ),
                                children: vec![Entry {
                                    current_kind: CursorKind::IntegerLiteral(CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/while.cpp".to_owned(),
                                            line: 2,
                                            col: 11
                                        },
                                        end_pos: Position {
                                            file_name: "tests/while.cpp".to_owned(),
                                            line: 2,
                                            col: 13
                                        }
                                    }),
                                    children: vec![]
                                }]
                            }]
                        },
                        Entry {
                            current_kind: CursorKind::WhileStatement(CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/while.cpp".to_owned(),
                                    line: 3,
                                    col: 3
                                },
                                end_pos: Position {
                                    file_name: "tests/while.cpp".to_owned(),
                                    line: 5,
                                    col: 4
                                }
                            }),
                            children: vec![
                                Entry {
                                    current_kind: CursorKind::BinaryOperator(CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/while.cpp".to_owned(),
                                            line: 3,
                                            col: 10
                                        },
                                        end_pos: Position {
                                            file_name: "tests/while.cpp".to_owned(),
                                            line: 3,
                                            col: 16
                                        }
                                    }),
                                    children: vec![
                                        Entry {
                                            current_kind: CursorKind::UnexposedExpression(
                                                "i".to_owned(),
                                                CodeSpan {
                                                    start_pos: Position {
                                                        file_name: "tests/while.cpp".to_owned(),
                                                        line: 3,
                                                        col: 10
                                                    },
                                                    end_pos: Position {
                                                        file_name: "tests/while.cpp".to_owned(),
                                                        line: 3,
                                                        col: 11
                                                    }
                                                }
                                            ),
                                            children: vec![Entry {
                                                current_kind:
                                                    CursorKind::DeclarationReferenceExpression(
                                                        "i".to_owned(),
                                                        CodeSpan {
                                                            start_pos: Position {
                                                                file_name: "tests/while.cpp"
                                                                    .to_owned(),
                                                                line: 3,
                                                                col: 10
                                                            },
                                                            end_pos: Position {
                                                                file_name: "tests/while.cpp"
                                                                    .to_owned(),
                                                                line: 3,
                                                                col: 11
                                                            }
                                                        }
                                                    ),
                                                children: vec![]
                                            }]
                                        },
                                        Entry {
                                            current_kind: CursorKind::IntegerLiteral(CodeSpan {
                                                start_pos: Position {
                                                    file_name: "tests/while.cpp".to_owned(),
                                                    line: 3,
                                                    col: 15
                                                },
                                                end_pos: Position {
                                                    file_name: "tests/while.cpp".to_owned(),
                                                    line: 3,
                                                    col: 16
                                                }
                                            }),
                                            children: vec![]
                                        }
                                    ]
                                },
                                Entry {
                                    current_kind: CursorKind::CompoundStatement(CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/while.cpp".to_owned(),
                                            line: 3,
                                            col: 18
                                        },
                                        end_pos: Position {
                                            file_name: "tests/while.cpp".to_owned(),
                                            line: 5,
                                            col: 4
                                        }
                                    }),
                                    children: vec![Entry {
                                        current_kind: CursorKind::UnaryOperator(CodeSpan {
                                            start_pos: Position {
                                                file_name: "tests/while.cpp".to_owned(),
                                                line: 4,
                                                col: 5
                                            },
                                            end_pos: Position {
                                                file_name: "tests/while.cpp".to_owned(),
                                                line: 4,
                                                col: 8
                                            }
                                        }),
                                        children: vec![Entry {
                                            current_kind:
                                                CursorKind::DeclarationReferenceExpression(
                                                    "i".to_owned(),
                                                    CodeSpan {
                                                        start_pos: Position {
                                                            file_name: "tests/while.cpp".to_owned(),
                                                            line: 4,
                                                            col: 7
                                                        },
                                                        end_pos: Position {
                                                            file_name: "tests/while.cpp".to_owned(),
                                                            line: 4,
                                                            col: 8
                                                        }
                                                    }
                                                ),
                                            children: vec![]
                                        }]
                                    }]
                                }
                            ]
                        }
                    ]
                }]
            }]
        }
    );
}

#[test]
fn test_do_while_loop() {
    let source = Source::from_file(
        "tests/do_while.cpp".to_owned(),
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
                current_kind: CursorKind::Function {
                    spelling: "f".to_owned(),
                    display_name: "f()".to_owned(),
                    code_span: CodeSpan {
                        start_pos: Position {
                            file_name: "tests/do_while.cpp".to_owned(),
                            line: 1,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/do_while.cpp".to_owned(),
                            line: 4,
                            col: 2
                        }
                    },
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::Void,
                    canonical_return_type: CursorType::Void
                },
                children: vec![Entry {
                    current_kind: CursorKind::CompoundStatement(CodeSpan {
                        start_pos: Position {
                            file_name: "tests/do_while.cpp".to_owned(),
                            line: 1,
                            col: 10
                        },
                        end_pos: Position {
                            file_name: "tests/do_while.cpp".to_owned(),
                            line: 4,
                            col: 2
                        }
                    }),
                    children: vec![Entry {
                        current_kind: CursorKind::DoStatement(CodeSpan {
                            start_pos: Position {
                                file_name: "tests/do_while.cpp".to_owned(),
                                line: 2,
                                col: 3
                            },
                            end_pos: Position {
                                file_name: "tests/do_while.cpp".to_owned(),
                                line: 3,
                                col: 18
                            }
                        }),
                        children: vec![
                            Entry {
                                current_kind: CursorKind::CompoundStatement(CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/do_while.cpp".to_owned(),
                                        line: 2,
                                        col: 6
                                    },
                                    end_pos: Position {
                                        file_name: "tests/do_while.cpp".to_owned(),
                                        line: 3,
                                        col: 4
                                    }
                                }),
                                children: vec![]
                            },
                            Entry {
                                current_kind: CursorKind::BoolLiteral(CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/do_while.cpp".to_owned(),
                                        line: 3,
                                        col: 12
                                    },
                                    end_pos: Position {
                                        file_name: "tests/do_while.cpp".to_owned(),
                                        line: 3,
                                        col: 17
                                    }
                                }),
                                children: vec![]
                            }
                        ]
                    }]
                }]
            }]
        }
    );
}
