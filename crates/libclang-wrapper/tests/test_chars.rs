use libclang_wrapper;
use libclang_wrapper::source::{
    CodeSpan, CursorKind, CursorType, DeclarationFromPHCMode, DiagnosticsMode, Entry, Position,
    Source, TUOptionsBuilder,
};

#[test]
fn test_chars() {
    let source = Source::from_file(
        "tests/chars.cpp".to_owned(),
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
            children: vec![Entry {
                current_kind: CursorKind::Function {
                    spelling: "char_stuff".to_owned(),
                    display_name: "char_stuff(unsigned char, signed char, char)".to_owned(),
                    code_span: CodeSpan {
                        start_pos: Position {
                            file_name: "tests/chars.cpp".to_owned(),
                            line: 1,
                            col: 1
                        },
                        end_pos: Position {
                            file_name: "tests/chars.cpp".to_owned(),
                            line: 1,
                            col: 56
                        }
                    },
                    cur_type: CursorType::FunctionProto,
                    return_type: CursorType::CharS,
                    canonical_return_type: CursorType::CharS
                },
                children: vec![
                    Entry {
                        current_kind: CursorKind::Parameter(
                            "a".to_owned(),
                            CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/chars.cpp".to_owned(),
                                    line: 1,
                                    col: 17
                                },
                                end_pos: Position {
                                    file_name: "tests/chars.cpp".to_owned(),
                                    line: 1,
                                    col: 32
                                }
                            },
                            CursorType::UnsignedChar
                        ),
                        children: vec![]
                    },
                    Entry {
                        current_kind: CursorKind::Parameter(
                            "b".to_owned(),
                            CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/chars.cpp".to_owned(),
                                    line: 1,
                                    col: 34
                                },
                                end_pos: Position {
                                    file_name: "tests/chars.cpp".to_owned(),
                                    line: 1,
                                    col: 47
                                }
                            },
                            CursorType::SignedChar
                        ),
                        children: vec![]
                    },
                    Entry {
                        current_kind: CursorKind::Parameter(
                            "c".to_owned(),
                            CodeSpan {
                                start_pos: Position {
                                    file_name: "tests/chars.cpp".to_owned(),
                                    line: 1,
                                    col: 49
                                },
                                end_pos: Position {
                                    file_name: "tests/chars.cpp".to_owned(),
                                    line: 1,
                                    col: 55
                                }
                            },
                            CursorType::CharS
                        ),
                        children: vec![]
                    }
                ]
            }]
        }
    );
}
