use libclang_wrapper;
use libclang_wrapper::source::{
    CodeSpan, CursorKind, CursorType, DeclarationFromPHCMode, DiagnosticsMode, Entry, Position,
    Source, TUOptionsBuilder,
};

#[test]
fn parse_single_function() {
    let source = Source::from_file(
        "tests/header.h".to_owned(),
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
                        spelling: "add".to_owned(),
                        display_name: "add(int, int)".to_owned(),
                        code_span: CodeSpan {
                            start_pos: Position {
                                file_name: "tests/header.h".to_owned(),
                                line: 4,
                                col: 1,
                            },
                            end_pos: Position {
                                file_name: "tests/header.h".to_owned(),
                                line: 4,
                                col: 22,
                            },
                        },
                        cur_type: CursorType::FunctionProto,
                        return_type: CursorType::Int,
                        canonical_return_type: CursorType::Int,
                    },
                    children: vec![
                        Entry {
                            current_kind: CursorKind::Parameter(
                                "a".to_owned(),
                                CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/header.h".to_owned(),
                                        line: 4,
                                        col: 9,
                                    },
                                    end_pos: Position {
                                        file_name: "tests/header.h".to_owned(),
                                        line: 4,
                                        col: 14,
                                    },
                                },
                                CursorType::Int,
                            ),
                            children: vec![],
                        },
                        Entry {
                            current_kind: CursorKind::Parameter(
                                "b".to_owned(),
                                CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/header.h".to_owned(),
                                        line: 4,
                                        col: 16,
                                    },
                                    end_pos: Position {
                                        file_name: "tests/header.h".to_owned(),
                                        line: 4,
                                        col: 21,
                                    },
                                },
                                CursorType::Int,
                            ),
                            children: vec![],
                        },
                    ],
                },
                Entry {
                    current_kind: CursorKind::Function {
                        spelling: "function_with_param".to_owned(),
                        display_name: "function_with_param(float &)".to_owned(),
                        code_span: CodeSpan {
                            start_pos: Position {
                                file_name: "tests/header.h".to_owned(),
                                line: 6,
                                col: 1,
                            },
                            end_pos: Position {
                                file_name: "tests/header.h".to_owned(),
                                line: 9,
                                col: 2,
                            },
                        },
                        cur_type: CursorType::FunctionProto,
                        return_type: CursorType::LValueReference,
                        canonical_return_type: CursorType::LValueReference,
                    },
                    children: vec![
                        Entry {
                            current_kind: CursorKind::Parameter(
                                "k".to_owned(),
                                CodeSpan {
                                    start_pos: Position {
                                        file_name: "tests/header.h".to_owned(),
                                        line: 6,
                                        col: 28,
                                    },
                                    end_pos: Position {
                                        file_name: "tests/header.h".to_owned(),
                                        line: 6,
                                        col: 36,
                                    },
                                },
                                CursorType::LValueReference,
                            ),
                            children: vec![],
                        },
                        Entry {
                            current_kind: CursorKind::CompoundStatement(CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/header.h".to_owned(),
                                    line: 6,
                                    col: 38,
                                },
                                end_pos: Position {
                                    file_name: "tests/header.h".to_owned(),
                                    line: 9,
                                    col: 2,
                                },
                            }),
                            children: vec![
                                Entry {
                                    current_kind: CursorKind::BinaryOperator(CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/header.h".to_owned(),
                                            line: 7,
                                            col: 3,
                                        },
                                        end_pos: Position {
                                            file_name: "tests/header.h".to_owned(),
                                            line: 7,
                                            col: 10,
                                        },
                                    }),
                                    children: vec![
                                        Entry {
                                            current_kind:
                                                CursorKind::DeclarationReferenceExpression(
                                                    "k".to_owned(),
                                                    CodeSpan {
                                                        start_pos: Position {
                                                            file_name: "tests/header.h".to_owned(),
                                                            line: 7,
                                                            col: 3,
                                                        },
                                                        end_pos: Position {
                                                            file_name: "tests/header.h".to_owned(),
                                                            line: 7,
                                                            col: 4,
                                                        },
                                                    },
                                                ),
                                            children: vec![],
                                        },
                                        Entry {
                                            current_kind: CursorKind::FloatLiteral(CodeSpan {
                                                start_pos: Position {
                                                    file_name: "tests/header.h".to_owned(),
                                                    line: 7,
                                                    col: 7,
                                                },
                                                end_pos: Position {
                                                    file_name: "tests/header.h".to_owned(),
                                                    line: 7,
                                                    col: 10,
                                                },
                                            }),
                                            children: vec![],
                                        },
                                    ],
                                },
                                Entry {
                                    current_kind: CursorKind::ReturnStatement(CodeSpan {
                                        start_pos: Position {
                                            file_name: "tests/header.h".to_owned(),
                                            line: 8,
                                            col: 3,
                                        },
                                        end_pos: Position {
                                            file_name: "tests/header.h".to_owned(),
                                            line: 8,
                                            col: 11,
                                        },
                                    }),
                                    children: vec![Entry {
                                        current_kind: CursorKind::DeclarationReferenceExpression(
                                            "k".to_owned(),
                                            CodeSpan {
                                                start_pos: Position {
                                                    file_name: "tests/header.h".to_owned(),
                                                    line: 8,
                                                    col: 10,
                                                },
                                                end_pos: Position {
                                                    file_name: "tests/header.h".to_owned(),
                                                    line: 8,
                                                    col: 11,
                                                },
                                            },
                                        ),
                                        children: vec![],
                                    }],
                                },
                            ],
                        },
                    ],
                },
            ],
        }
    );
}
