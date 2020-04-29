use libclang_wrapper;
use libclang_wrapper::source::{
    CompilationDatabase, DeclarationFromPHCMode, DiagnosticsMode, Parsed, Source, TUOptionsBuilder,
};
use std::path::Path;

#[test]
fn test_compilation_database() {
    let compile_database =
        CompilationDatabase::new(Path::new("tests/test_compile_commands.json")).unwrap();
    assert_eq!(
        compile_database.commands,
        vec![
            Parsed {
                args: vec![
                    "/usr/bin/clang++-10".to_owned(),
                    "-fno-limit-debug-info".to_owned(),
                    "-fPIC".to_owned(),
                    "-x".to_owned(),
                    "c++".to_owned(),
                    "-std=c++14".to_owned()
                ],
                file: "./tests/class.h".to_owned()
            },
            Parsed {
                args: vec![
                    "/usr/bin/clang++-10".to_owned(),
                    "-fno-limit-debug-info".to_owned(),
                    "-fPIC".to_owned(),
                    "-std=gnu++14".to_owned()
                ],
                file: "./tests/header.h".to_owned()
            }
        ]
    );
}

#[test]
fn test_parsing_with_compilation_database() {
    let compile_database =
        CompilationDatabase::new(Path::new("tests/test_compile_commands.json")).unwrap();
    let source = Source::from_compilation_database(
        DeclarationFromPHCMode::Exclude,
        DiagnosticsMode::Enabled,
        compile_database,
        TUOptionsBuilder::new(),
    )
    .unwrap();
    let (success, errors): (Vec<_>, Vec<_>) = source
        .translation_units
        .into_iter()
        .partition(Result::is_ok);
    assert_eq!(success.len(), 2);
    assert_eq!(errors.len(), 0);
}
