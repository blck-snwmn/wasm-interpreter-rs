use crate::ast::{self, module, section};
pub struct FunctionInstance {
    func_type: ast::wasm_type::FunctionType,
    // typeindex は type をすでに持っているのでなしでOK
    code: ast::section::Code,
}

impl FunctionInstance {
    pub fn new(ft: ast::wasm_type::FunctionType, c: ast::section::Code) -> Self {
        FunctionInstance {
            func_type: ft,
            code: c,
        }
    }
}
