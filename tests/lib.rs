use clang_sys::*;
use libclang_wrapper;
use libclang_wrapper::index;

//#[cfg(feature = "runtime")]
//#[test]
//fn test() {
//    load().unwrap();
//    parse();
//    unload().unwrap();
//}

#[cfg(not(feature = "runtime"))]
#[test]
fn create_index_test() {
    let index = index::Index::new(
        "tests/header.h".to_owned(),
        index::DeclarationFromPHCMode::Exclude,
        index::DiagnosticsMode::Enabled,
        index::TUOptionsBuilder::new(),
    );
}

#[test]
fn test_support() {
    let clang = support::Clang::find(None, &[]).unwrap();
    println!("{:?}", clang);
}
