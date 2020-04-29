use libclang_wrapper;
use libclang_wrapper::source::{
    AccessSpecifierType, CodeSpan, CursorKind, CursorType, DeclarationFromPHCMode, DiagnosticsMode,
    Entry, Position, Source, TUOptionsBuilder,
};

#[test]
fn test_type_aliases_and_typedefs() {
    let source = Source::from_file(
        "tests/type_aliases_and_typedefs.cpp".to_owned(),
        DeclarationFromPHCMode::Exclude,
        DiagnosticsMode::Enabled,
        vec!["-std=c++14".to_owned()],
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
                    current_kind: CursorKind::Struct(
                        "M".to_owned(),
                        CodeSpan {
                            start_pos: Position {
                                file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                line: 3,
                                col: 1
                            },
                            end_pos: Position {
                                file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                line: 3,
                                col: 11
                            }
                        },
                        AccessSpecifierType::Invalid
                    ),
                    children: vec![]
                },
                Entry {
                    current_kind: CursorKind::TypeAlias(
                        "my_integer".to_owned(),
                        CursorType::Int,
                        CodeSpan {
                            start_pos: Position {
                                file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                line: 5,
                                col: 1
                            },
                            end_pos: Position {
                                file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                line: 5,
                                col: 32
                            }
                        },
                        AccessSpecifierType::Invalid
                    ),
                    children: vec![
                        Entry {
                            current_kind: CursorKind::NamespaceReference(
                                "std".to_owned(),
                                CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                        line: 5,
                                        col: 20
                                    },
                                    end_pos: Position {
                                        file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                        line: 5,
                                        col: 23
                                    }
                                }
                            ),
                            children: vec![]
                        },
                        Entry {
                            current_kind: CursorKind::TypeReference(
                                "int32_t".to_owned(),
                                CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                        line: 5,
                                        col: 25
                                    },
                                    end_pos: Position {
                                        file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                        line: 5,
                                        col: 32
                                    }
                                }
                            ),
                            children: vec![]
                        }
                    ]
                },
                Entry {
                    current_kind: CursorKind::Typedef(
                        "TM".to_owned(),
                        CursorType::Record,
                        CodeSpan {
                            start_pos: Position {
                                file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                line: 7,
                                col: 1
                            },
                            end_pos: Position {
                                file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                line: 7,
                                col: 13
                            }
                        },
                        AccessSpecifierType::Invalid
                    ),
                    children: vec![Entry {
                        current_kind: CursorKind::TypeReference(
                            "struct M".to_owned(),
                            CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                    line: 7,
                                    col: 9
                                },
                                end_pos: Position {
                                    file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                    line: 7,
                                    col: 10
                                }
                            }
                        ),
                        children: vec![]
                    }]
                },
                Entry {
                    current_kind: CursorKind::TypeAlias(
                        "my_m".to_owned(),
                        CursorType::Record,
                        CodeSpan {
                            start_pos: Position {
                                file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                line: 9,
                                col: 1
                            },
                            end_pos: Position {
                                file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                line: 9,
                                col: 15
                            }
                        },
                        AccessSpecifierType::Invalid
                    ),
                    children: vec![Entry {
                        current_kind: CursorKind::TypeReference(
                            "struct M".to_owned(),
                            CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                    line: 9,
                                    col: 14
                                },
                                end_pos: Position {
                                    file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                    line: 9,
                                    col: 15
                                }
                            }
                        ),
                        children: vec![]
                    }]
                },
                Entry {
                    current_kind: CursorKind::TypeAlias(
                        "alias_alias".to_owned(),
                        CursorType::Record,
                        CodeSpan {
                            start_pos: Position {
                                file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                line: 11,
                                col: 1
                            },
                            end_pos: Position {
                                file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                line: 11,
                                col: 25
                            }
                        },
                        AccessSpecifierType::Invalid
                    ),
                    children: vec![Entry {
                        current_kind: CursorKind::TypeReference(
                            "my_m".to_owned(),
                            CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                    line: 11,
                                    col: 21
                                },
                                end_pos: Position {
                                    file_name: "tests/type_aliases_and_typedefs.cpp".to_owned(),
                                    line: 11,
                                    col: 25
                                }
                            }
                        ),
                        children: vec![]
                    }]
                }
            ]
        }
    );
}
