use super::value;
use crate::ast::{self, module, section};

pub struct FunctionInstance {
    pub func_type: ast::wasm_type::FunctionType,
    // typeindex は type をすでに持っているのでなしでOK
    pub code: ast::section::Code,
}

impl FunctionInstance {
    pub fn new(ft: ast::wasm_type::FunctionType, c: ast::section::Code) -> Self {
        FunctionInstance {
            func_type: ft,
            code: c,
        }
    }
}

pub struct ExportInstance {
    name: String,
    value: value::ExternVal,
}

impl ExportInstance {
    pub fn new(name: String, value: value::ExternVal) -> Self {
        Self { name, value }
    }
}
