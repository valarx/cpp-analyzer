use libclang_wrapper;
use libclang_wrapper::source::{
    CodeSpan, CursorKind, CursorType, DeclarationFromPHCMode, DiagnosticsMode, Entry, Position,
    Source, TUOptionsBuilder,
};

#[test]
fn test_classic_for_loop() {
    let source = Source::from_file(
        "tests/for.cpp".to_owned(),
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
                            file_name: "tests/for.cpp".to_owned(),
                            line: 1,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/for.cpp".to_owned(),
                            line: 5,
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
                            file_name: "tests/for.cpp".to_owned(),
                            line: 1,
                            col: 10
                        },
                        end_pos: Position {
                            file_name: "tests/for.cpp".to_owned(),
                            line: 5,
                            col: 2
                        }
                    }),
                    children: vec![Entry {
                        current_kind: CursorKind::ForStatement(CodeSpan {
                            start_pos: Position {
                                file_name: "tests/for.cpp".to_owned(),
                                line: 2,
                                col: 3
                            },
                            end_pos: Position {
                                file_name: "tests/for.cpp".to_owned(),
                                line: 4,
                                col: 4
                            }
                        }),
                        children: vec![
                            Entry {
                                current_kind: CursorKind::DeclarationStatement(CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/for.cpp".to_owned(),
                                        line: 2,
                                        col: 8
                                    },
                                    end_pos: Position {
                                        file_name: "tests/for.cpp".to_owned(),
                                        line: 2,
                                        col: 18
                                    }
                                }),
                                children: vec![Entry {
                                    current_kind: CursorKind::Variable(
                                        "i".to_owned(),
                                        CodeSpan {
                                            start_pos: Position {
                                                file_name: "tests/for.cpp".to_owned(),
                                                line: 2,
                                                col: 8
                                            },
                                            end_pos: Position {
                                                file_name: "tests/for.cpp".to_owned(),
                                                line: 2,
                                                col: 17
                                            }
                                        },
                                        CursorType::Int
                                    ),
                                    children: vec![Entry {
                                        current_kind: CursorKind::IntegerLiteral(CodeSpan {
                                            start_pos: Position {
                                                file_name: "tests/for.cpp".to_owned(),
                                                line: 2,
                                                col: 16
                                            },
                                            end_pos: Position {
                                                file_name: "tests/for.cpp".to_owned(),
                                                line: 2,
                                                col: 17
                                            }
                                        }),
                                        children: vec![]
                                    }]
                                }]
                            },
                            Entry {
                                current_kind: CursorKind::BinaryOperator(CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/for.cpp".to_owned(),
                                        line: 2,
                                        col: 19
                                    },
                                    end_pos: Position {
                                        file_name: "tests/for.cpp".to_owned(),
                                        line: 2,
                                        col: 24
                                    }
                                }),
                                children: vec![
                                    Entry {
                                        current_kind: CursorKind::UnexposedExpression(
                                            "i".to_owned(),
                                            CodeSpan {
                                                start_pos: Position {
                                                    file_name: "tests/for.cpp".to_owned(),
                                                    line: 2,
                                                    col: 19
                                                },
                                                end_pos: Position {
                                                    file_name: "tests/for.cpp".to_owned(),
                                                    line: 2,
                                                    col: 20
                                                }
                                            }
                                        ),
                                        children: vec![Entry {
                                            current_kind:
                                                CursorKind::DeclarationReferenceExpression(
                                                    "i".to_owned(),
                                                    CodeSpan {
                                                        start_pos: Position {
                                                            file_name: "tests/for.cpp".to_owned(),
                                                            line: 2,
                                                            col: 19
                                                        },
                                                        end_pos: Position {
                                                            file_name: "tests/for.cpp".to_owned(),
                                                            line: 2,
                                                            col: 20
                                                        }
                                                    }
                                                ),
                                            children: vec![]
                                        }]
                                    },
                                    Entry {
                                        current_kind: CursorKind::IntegerLiteral(CodeSpan {
                                            start_pos: Position {
                                                file_name: "tests/for.cpp".to_owned(),
                                                line: 2,
                                                col: 23
                                            },
                                            end_pos: Position {
                                                file_name: "tests/for.cpp".to_owned(),
                                                line: 2,
                                                col: 24
                                            }
                                        }),
                                        children: vec![]
                                    }
                                ]
                            },
                            Entry {
                                current_kind: CursorKind::UnaryOperator(CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/for.cpp".to_owned(),
                                        line: 2,
                                        col: 26
                                    },
                                    end_pos: Position {
                                        file_name: "tests/for.cpp".to_owned(),
                                        line: 2,
                                        col: 29
                                    }
                                }),
                                children: vec![Entry {
                                    current_kind: CursorKind::DeclarationReferenceExpression(
                                        "i".to_owned(),
                                        CodeSpan {
                                            start_pos: Position {
                                                file_name: "tests/for.cpp".to_owned(),
                                                line: 2,
                                                col: 28
                                            },
                                            end_pos: Position {
                                                file_name: "tests/for.cpp".to_owned(),
                                                line: 2,
                                                col: 29
                                            }
                                        }
                                    ),
                                    children: vec![]
                                }]
                            },
                            Entry {
                                current_kind: CursorKind::CompoundStatement(CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/for.cpp".to_owned(),
                                        line: 2,
                                        col: 31
                                    },
                                    end_pos: Position {
                                        file_name: "tests/for.cpp".to_owned(),
                                        line: 4,
                                        col: 4
                                    }
                                }),
                                children: vec![Entry {
                                    current_kind: CursorKind::ContinueStatement(CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/for.cpp".to_owned(),
                                            line: 3,
                                            col: 5
                                        },
                                        end_pos: Position {
                                            file_name: "tests/for.cpp".to_owned(),
                                            line: 3,
                                            col: 13
                                        }
                                    }),
                                    children: vec![]
                                }]
                            }
                        ]
                    }]
                }]
            }]
        }
    );
}

#[test]
fn test_for_no_initializer() {
    let source = Source::from_file(
        "tests/for_no_init.cpp".to_owned(),
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
                            file_name: "tests/for_no_init.cpp".to_owned(),
                            line: 1,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/for_no_init.cpp".to_owned(),
                            line: 5,
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
                            file_name: "tests/for_no_init.cpp".to_owned(),
                            line: 1,
                            col: 10
                        },
                        end_pos: Position {
                            file_name: "tests/for_no_init.cpp".to_owned(),
                            line: 5,
                            col: 2
                        }
                    }),
                    children: vec![
                        Entry {
                            current_kind: CursorKind::DeclarationStatement(CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/for_no_init.cpp".to_owned(),
                                    line: 2,
                                    col: 3
                                },
                                end_pos: Position {
                                    file_name: "tests/for_no_init.cpp".to_owned(),
                                    line: 2,
                                    col: 13
                                }
                            }),
                            children: vec![Entry {
                                current_kind: CursorKind::Variable(
                                    "i".to_owned(),
                                    CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/for_no_init.cpp".to_owned(),
                                            line: 2,
                                            col: 3
                                        },
                                        end_pos: Position {
                                            file_name: "tests/for_no_init.cpp".to_owned(),
                                            line: 2,
                                            col: 12
                                        }
                                    },
                                    CursorType::Int
                                ),
                                children: vec![Entry {
                                    current_kind: CursorKind::IntegerLiteral(CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/for_no_init.cpp".to_owned(),
                                            line: 2,
                                            col: 11
                                        },
                                        end_pos: Position {
                                            file_name: "tests/for_no_init.cpp".to_owned(),
                                            line: 2,
                                            col: 12
                                        }
                                    }),
                                    children: vec![]
                                }]
                            }]
                        },
                        Entry {
                            current_kind: CursorKind::ForStatement(CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/for_no_init.cpp".to_owned(),
                                    line: 3,
                                    col: 3
                                },
                                end_pos: Position {
                                    file_name: "tests/for_no_init.cpp".to_owned(),
                                    line: 4,
                                    col: 4
                                }
                            }),
                            children: vec![
                                Entry {
                                    current_kind: CursorKind::BinaryOperator(CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/for_no_init.cpp".to_owned(),
                                            line: 3,
                                            col: 10
                                        },
                                        end_pos: Position {
                                            file_name: "tests/for_no_init.cpp".to_owned(),
                                            line: 3,
                                            col: 15
                                        }
                                    }),
                                    children: vec![
                                        Entry {
                                            current_kind: CursorKind::UnexposedExpression(
                                                "i".to_owned(),
                                                CodeSpan {
                                                    start_pos: Position {
                                                        file_name: "tests/for_no_init.cpp"
                                                            .to_owned(),
                                                        line: 3,
                                                        col: 10
                                                    },
                                                    end_pos: Position {
                                                        file_name: "tests/for_no_init.cpp"
                                                            .to_owned(),
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
                                                                file_name: "tests/for_no_init.cpp"
                                                                    .to_owned(),
                                                                line: 3,
                                                                col: 10
                                                            },
                                                            end_pos: Position {
                                                                file_name: "tests/for_no_init.cpp"
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
                                                    file_name: "tests/for_no_init.cpp".to_owned(),
                                                    line: 3,
                                                    col: 14
                                                },
                                                end_pos: Position {
                                                    file_name: "tests/for_no_init.cpp".to_owned(),
                                                    line: 3,
                                                    col: 15
                                                }
                                            }),
                                            children: vec![]
                                        }
                                    ]
                                },
                                Entry {
                                    current_kind: CursorKind::UnaryOperator(CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/for_no_init.cpp".to_owned(),
                                            line: 3,
                                            col: 17
                                        },
                                        end_pos: Position {
                                            file_name: "tests/for_no_init.cpp".to_owned(),
                                            line: 3,
                                            col: 20
                                        }
                                    }),
                                    children: vec![Entry {
                                        current_kind: CursorKind::DeclarationReferenceExpression(
                                            "i".to_owned(),
                                            CodeSpan {
                                                start_pos: Position {
                                                    file_name: "tests/for_no_init.cpp".to_owned(),
                                                    line: 3,
                                                    col: 19
                                                },
                                                end_pos: Position {
                                                    file_name: "tests/for_no_init.cpp".to_owned(),
                                                    line: 3,
                                                    col: 20
                                                }
                                            }
                                        ),
                                        children: vec![]
                                    }]
                                },
                                Entry {
                                    current_kind: CursorKind::CompoundStatement(CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/for_no_init.cpp".to_owned(),
                                            line: 3,
                                            col: 22
                                        },
                                        end_pos: Position {
                                            file_name: "tests/for_no_init.cpp".to_owned(),
                                            line: 4,
                                            col: 4
                                        }
                                    }),
                                    children: vec![]
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
fn test_for_range_based() {
    let source = Source::from_file(
        "tests/for_range_based.cpp".to_owned(),
        DeclarationFromPHCMode::Exclude,
        DiagnosticsMode::Enabled,
        vec!["-std=c+14".to_owned()],
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
                    display_name: "f(const std::vector<int> &)".to_owned(),
                    code_span: CodeSpan {
                        start_pos: Position {
                            file_name: "tests/for_range_based.cpp".to_owned(),
                            line: 3,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/for_range_based.cpp".to_owned(),
                            line: 6,
                            col: 2
                        }
                    },
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::Void,
                    canonical_return_type: CursorType::Void
                },
                children: vec![
                    Entry {
                        current_kind: CursorKind::Parameter(
                            "a".to_owned(),
                            CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/for_range_based.cpp".to_owned(),
                                    line: 3,
                                    col: 8
                                },
                                end_pos: Position {
                                    file_name: "tests/for_range_based.cpp".to_owned(),
                                    line: 3,
                                    col: 33
                                }
                            },
                            CursorType::LValueReference
                        ),
                        children: vec![
                            Entry {
                                current_kind: CursorKind::NamespaceReference(
                                    "std".to_owned(),
                                    CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/for_range_based.cpp".to_owned(),
                                            line: 3,
                                            col: 14
                                        },
                                        end_pos: Position {
                                            file_name: "tests/for_range_based.cpp".to_owned(),
                                            line: 3,
                                            col: 17
                                        }
                                    }
                                ),
                                children: vec![]
                            },
                            Entry {
                                current_kind: CursorKind::TemplateReference(
                                    "vector".to_owned(),
                                    CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/for_range_based.cpp".to_owned(),
                                            line: 3,
                                            col: 19
                                        },
                                        end_pos: Position {
                                            file_name: "tests/for_range_based.cpp".to_owned(),
                                            line: 3,
                                            col: 25
                                        }
                                    }
                                ),
                                children: vec![]
                            }
                        ]
                    },
                    Entry {
                        current_kind: CursorKind::CompoundStatement(CodeSpan {
                            start_pos: Position {
                                file_name: "tests/for_range_based.cpp".to_owned(),
                                line: 3,
                                col: 35
                            },
                            end_pos: Position {
                                file_name: "tests/for_range_based.cpp".to_owned(),
                                line: 6,
                                col: 2
                            }
                        }),
                        children: vec![Entry {
                            current_kind: CursorKind::RangeBasedForStatement(CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/for_range_based.cpp".to_owned(),
                                    line: 4,
                                    col: 3
                                },
                                end_pos: Position {
                                    file_name: "tests/for_range_based.cpp".to_owned(),
                                    line: 5,
                                    col: 4
                                }
                            }),
                            children: vec![
                                Entry {
                                    current_kind: CursorKind::Variable(
                                        "val".to_owned(),
                                        CodeSpan {
                                            start_pos: Position {
                                                file_name: "tests/for_range_based.cpp".to_owned(),
                                                line: 4,
                                                col: 8
                                            },
                                            end_pos: Position {
                                                file_name: "tests/for_range_based.cpp".to_owned(),
                                                line: 4,
                                                col: 25
                                            }
                                        },
                                        CursorType::LValueReference
                                    ),
                                    children: vec![Entry {
                                        current_kind: CursorKind::CallExpression(
                                            "operator*".to_owned(),
                                            CodeSpan {
                                                start_pos: Position {
                                                    file_name: "tests/for_range_based.cpp"
                                                        .to_owned(),
                                                    line: 4,
                                                    col: 24
                                                },
                                                end_pos: Position {
                                                    file_name: "tests/for_range_based.cpp"
                                                        .to_owned(),
                                                    line: 4,
                                                    col: 25
                                                }
                                            },
                                        ),
                                        children: vec![
                                            Entry {
                                                current_kind: CursorKind::UnexposedExpression(
                                                    "__begin1".to_owned(),
                                                    CodeSpan {
                                                        start_pos: Position {
                                                            file_name: "tests/for_range_based.cpp"
                                                                .to_owned(),
                                                            line: 4,
                                                            col: 24
                                                        },
                                                        end_pos: Position {
                                                            file_name: "tests/for_range_based.cpp"
                                                                .to_owned(),
                                                            line: 4,
                                                            col: 25
                                                        }
                                                    }
                                                ),
                                                children: vec![Entry {
                                                    current_kind:
                                                        CursorKind::DeclarationReferenceExpression(
                                                            "__begin1".to_owned(),
                                                            CodeSpan {
                                                                start_pos: Position {
                                                                    file_name:
                                                                        "tests/for_range_based.cpp"
                                                                            .to_owned(),
                                                                    line: 4,
                                                                    col: 24
                                                                },
                                                                end_pos: Position {
                                                                    file_name:
                                                                        "tests/for_range_based.cpp"
                                                                            .to_owned(),
                                                                    line: 4,
                                                                    col: 25
                                                                }
                                                            }
                                                        ),
                                                    children: vec![]
                                                }]
                                            },
                                            Entry {
                                                current_kind: CursorKind::UnexposedExpression(
                                                    "operator*".to_owned(),
                                                    CodeSpan {
                                                        start_pos: Position {
                                                            file_name: "tests/for_range_based.cpp"
                                                                .to_owned(),
                                                            line: 4,
                                                            col: 24
                                                        },
                                                        end_pos: Position {
                                                            file_name: "tests/for_range_based.cpp"
                                                                .to_owned(),
                                                            line: 4,
                                                            col: 25
                                                        }
                                                    }
                                                ),
                                                children: vec![Entry {
                                                    current_kind:
                                                        CursorKind::DeclarationReferenceExpression(
                                                            "operator*".to_owned(),
                                                            CodeSpan {
                                                                start_pos: Position {
                                                                    file_name:
                                                                        "tests/for_range_based.cpp"
                                                                            .to_owned(),
                                                                    line: 4,
                                                                    col: 24
                                                                },
                                                                end_pos: Position {
                                                                    file_name:
                                                                        "tests/for_range_based.cpp"
                                                                            .to_owned(),
                                                                    line: 4,
                                                                    col: 25
                                                                }
                                                            }
                                                        ),
                                                    children: vec![]
                                                }]
                                            }
                                        ]
                                    }]
                                },
                                Entry {
                                    current_kind: CursorKind::DeclarationReferenceExpression(
                                        "a".to_owned(),
                                        CodeSpan {
                                            start_pos: Position {
                                                file_name: "tests/for_range_based.cpp".to_owned(),
                                                line: 4,
                                                col: 26
                                            },
                                            end_pos: Position {
                                                file_name: "tests/for_range_based.cpp".to_owned(),
                                                line: 4,
                                                col: 27
                                            }
                                        }
                                    ),
                                    children: vec![]
                                },
                                Entry {
                                    current_kind: CursorKind::CompoundStatement(CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/for_range_based.cpp".to_owned(),
                                            line: 4,
                                            col: 29
                                        },
                                        end_pos: Position {
                                            file_name: "tests/for_range_based.cpp".to_owned(),
                                            line: 5,
                                            col: 4
                                        }
                                    }),
                                    children: vec![]
                                }
                            ]
                        }]
                    }
                ]
            }]
        }
    );
}
