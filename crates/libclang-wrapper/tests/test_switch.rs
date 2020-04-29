use libclang_wrapper;
use libclang_wrapper::source::{
    CodeSpan, CursorKind, CursorType, DeclarationFromPHCMode, DiagnosticsMode, Entry, Position,
    Source, TUOptionsBuilder,
};

#[test]
fn test_parsing_switch_conditions() {
    let source = Source::from_file(
        "tests/switch.cpp".to_owned(),
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
            children: vec![
                Entry {
                    current_kind: CursorKind::Function {
                        spelling: "func".to_owned(),
                        display_name: "func(int)".to_owned(),
                        code_span: CodeSpan {
                            start_pos: Position {
                                file_name: "tests/switch.cpp".to_owned(),
                                line: 1,
                                col: 1
                            },
                            end_pos: Position {
                                file_name: "tests/switch.cpp".to_owned(),
                                line: 10,
                                col: 2
                            }
                        },
                        cur_type: CursorType::FunctionProto,
                        return_type: CursorType::Int,
                        canonical_return_type: CursorType::Int
                    },
                    children: vec![
                        Entry {
                            current_kind: CursorKind::Parameter(
                                "a".to_owned(),
                                CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/switch.cpp".to_owned(),
                                        line: 1,
                                        col: 10
                                    },
                                    end_pos: Position {
                                        file_name: "tests/switch.cpp".to_owned(),
                                        line: 1,
                                        col: 15
                                    }
                                },
                                CursorType::Int
                            ),
                            children: vec![]
                        },
                        Entry {
                            current_kind: CursorKind::CompoundStatement(
                                CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/switch.cpp".to_owned(),
                                        line: 1,
                                        col: 17
                                    },
                                    end_pos: Position {
                                        file_name: "tests/switch.cpp".to_owned(),
                                        line: 10,
                                        col: 2
                                    }
                                }
                            ),
                            children: vec![
                                Entry {
                                    current_kind: CursorKind::SwitchStatement(
                                        CodeSpan {
                                            start_pos: Position {
                                                file_name: "tests/switch.cpp".to_owned(),
                                                line: 2,
                                                col: 3
                                            },
                                            end_pos: Position {
                                                file_name: "tests/switch.cpp".to_owned(),
                                                line: 8,
                                                col: 4
                                            }
                                        }
                                    ),
                                    children: vec![
                                        Entry {
                                            current_kind: CursorKind::UnexposedExpression(
                                                "a".to_owned(),
                                                CodeSpan {
                                                    start_pos: Position {
                                                        file_name: "tests/switch.cpp".to_owned(),
                                                        line: 2,
                                                        col: 11
                                                    },
                                                    end_pos: Position {
                                                        file_name: "tests/switch.cpp".to_owned(),
                                                        line: 2,
                                                        col: 12
                                                    }
                                                },
                                            ),
                                            children: vec![
                                                Entry {
                                                    current_kind: CursorKind::DeclarationReferenceExpression(
                                                        "a".to_owned(),
                                                        CodeSpan {
                                                            start_pos: Position {
                                                                file_name: "tests/switch.cpp".to_owned(),
                                                                line: 2,
                                                                col: 11
                                                            },
                                                            end_pos: Position {
                                                                file_name: "tests/switch.cpp".to_owned(),
                                                                line: 2,
                                                                col: 12
                                                            }
                                                        }
                                                    ),
                                                    children: vec![]
                                                }
                                            ]
                                        },
                                        Entry {
                                            current_kind: CursorKind::CompoundStatement(
                                                CodeSpan {
                                                    start_pos: Position {
                                                        file_name: "tests/switch.cpp".to_owned(),
                                                        line: 2,
                                                        col: 14
                                                    },
                                                    end_pos: Position {
                                                        file_name: "tests/switch.cpp".to_owned(),
                                                        line: 8,
                                                        col: 4
                                                    }
                                                }
                                            ),
                                            children: vec![
                                                Entry {
                                                    current_kind: CursorKind::CaseStatement(
                                                        CodeSpan {
                                                            start_pos: Position {
                                                                file_name: "tests/switch.cpp".to_owned(),
                                                                line: 3,
                                                                col: 3
                                                            },
                                                            end_pos: Position {
                                                                file_name: "tests/switch.cpp".to_owned(),
                                                                line: 4,
                                                                col: 13
                                                            }
                                                        }
                                                    ),
                                                    children: vec![
                                                        Entry {
                                                            current_kind: CursorKind::IntegerLiteral(
                                                                CodeSpan {
                                                                    start_pos: Position {
                                                                        file_name: "tests/switch.cpp".to_owned(),
                                                                        line: 3,
                                                                        col: 8
                                                                    },
                                                                    end_pos: Position {
                                                                        file_name: "tests/switch.cpp".to_owned(),
                                                                        line: 3,
                                                                        col: 9
                                                                    }
                                                                }
                                                            ),
                                                            children: vec![]
                                                        },
                                                        Entry {
                                                            current_kind: CursorKind::ReturnStatement(
                                                                CodeSpan {
                                                                    start_pos: Position {
                                                                        file_name: "tests/switch.cpp".to_owned(),
                                                                        line: 4,
                                                                        col: 5
                                                                    },
                                                                    end_pos: Position {
                                                                        file_name: "tests/switch.cpp".to_owned(),
                                                                        line: 4,
                                                                        col: 13
                                                                    }
                                                                }
                                                            ),
                                                            children: vec![
                                                                Entry {
                                                                    current_kind: CursorKind::UnexposedExpression(
                                                                        "a".to_owned(),
                                                                        CodeSpan {
                                                                            start_pos: Position {
                                                                                file_name: "tests/switch.cpp".to_owned(),
                                                                                line: 4,
                                                                                col: 12
                                                                            },
                                                                            end_pos: Position {
                                                                                file_name: "tests/switch.cpp".to_owned(),
                                                                                line: 4,
                                                                                col: 13
                                                                            }
                                                                        }
                                                                    ),
                                                                    children: vec![
                                                                        Entry {
                                                                            current_kind:
                                                                                CursorKind::DeclarationReferenceExpression(
                                                                                    "a".to_owned(),
                                                                                    CodeSpan {
                                                                                        start_pos: Position {
                                                                                            file_name: "tests/switch.cpp"
                                                                                                .to_owned(),
                                                                                            line: 4,
                                                                                            col: 12
                                                                                        },
                                                                                        end_pos: Position {
                                                                                            file_name: "tests/switch.cpp"
                                                                                                .to_owned(),
                                                                                            line: 4,
                                                                                            col: 13
                                                                                        }
                                                                                    }
                                                                                ),
                                                                            children: vec![]
                                                                        }
                                                                    ]
                                                                }
                                                            ]
                                                        }
                                                    ]
                                                },
                                                Entry {
                                                    current_kind: CursorKind::BreakStatement (
                                                        CodeSpan {
                                                            start_pos: Position {
                                                                file_name: "tests/switch.cpp"
                                                                    .to_owned(),
                                                                line: 5,
                                                                col: 5
                                                            },
                                                            end_pos: Position {
                                                                file_name: "tests/switch.cpp"
                                                                    .to_owned(),
                                                                line: 5,
                                                                col: 10
                                                            }
                                                        }
                                                    ),
                                                    children: vec![]
                                                },
                                                Entry {
                                                    current_kind: CursorKind::DefaultStatement (
                                                        CodeSpan {
                                                            start_pos: Position {
                                                                file_name: "tests/switch.cpp"
                                                                    .to_owned(),
                                                                line: 6,
                                                                col: 3
                                                            },
                                                            end_pos: Position {
                                                                file_name: "tests/switch.cpp"
                                                                    .to_owned(),
                                                                line: 7,
                                                                col: 10
                                                            }
                                                        }
                                                    ),
                                                    children: vec![
                                                        Entry {
                                                            current_kind: CursorKind::BreakStatement (
                                                                CodeSpan {
                                                                    start_pos: Position {
                                                                        file_name: "tests/switch.cpp"
                                                                            .to_owned(),
                                                                        line: 7,
                                                                        col: 5
                                                                    },
                                                                    end_pos: Position {
                                                                        file_name: "tests/switch.cpp"
                                                                            .to_owned(),
                                                                        line: 7,
                                                                        col: 10
                                                                    }
                                                                }
                                                            ),
                                                            children: vec![]
                                                        }
                                                    ]
                                                }
                                            ]
                                        }
                                    ]
                                },
                                Entry {
                                    current_kind: CursorKind::ReturnStatement(
                                        CodeSpan {
                                            start_pos: Position {
                                                file_name: "tests/switch.cpp"
                                                    .to_owned(),
                                                line: 9,
                                                col: 3
                                            },
                                            end_pos: Position {
                                                file_name: "tests/switch.cpp"
                                                    .to_owned(),
                                                line: 9,
                                                col: 11
                                            }
                                        }
                                    ),
                                    children: vec![
                                        Entry {
                                            current_kind: CursorKind::IntegerLiteral(
                                                CodeSpan {
                                                    start_pos: Position {
                                                        file_name: "tests/switch.cpp"
                                                            .to_owned(),
                                                        line: 9,
                                                        col: 10
                                                    },
                                                    end_pos: Position {
                                                        file_name: "tests/switch.cpp"
                                                            .to_owned(),
                                                        line: 9,
                                                        col: 11
                                                    }
                                                }
                                            ),
                                            children: vec![]
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ]
        }
    );
}
