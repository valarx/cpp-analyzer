use clang_sys::*;

#[test]
fn test_support() {
    clang_sys::load().unwrap();
    let _ = clang_sys::get_library().unwrap();
    let clang = support::Clang::find(None, &[]).unwrap();
    println!("{:?}", clang);
    clang_sys::unload().unwrap();
}
