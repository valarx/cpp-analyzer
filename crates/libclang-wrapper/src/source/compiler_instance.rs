pub struct CompilerInstance {}

impl CompilerInstance {
    pub fn new() -> CompilerInstance {
        clang_sys::load().unwrap();
        clang_sys::get_library().unwrap();
        CompilerInstance {}
    }
}

impl Drop for CompilerInstance {
    fn drop(&mut self) {
        clang_sys::unload().unwrap();
    }
}
