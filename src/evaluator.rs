use crate::{
    ast::{module, section},
    object::instance,
};
struct Store {}
struct Executor {
    store: Store,
    stack: Vec<u128>,
}

impl Executor {
    fn new(module: module::Module) -> Self {
        let mut func = None;
        let mut code = None;
        let mut typ = None;
        for s in module.sections {
            match s.payload_data {
                crate::ast::section::SectionData::Custom(_) => {} // do noting
                crate::ast::section::SectionData::Type(t) => typ = Some(t),
                crate::ast::section::SectionData::Import => {}
                crate::ast::section::SectionData::Function(f) => func = Some(f),
                crate::ast::section::SectionData::Table => {}
                crate::ast::section::SectionData::Memory => {}
                crate::ast::section::SectionData::Global => {}
                crate::ast::section::SectionData::Export => {}
                crate::ast::section::SectionData::Start => {}
                crate::ast::section::SectionData::Element => {}
                crate::ast::section::SectionData::Code(c) => code = Some(c),
                crate::ast::section::SectionData::Data => {}
                crate::ast::section::SectionData::DataCount => {}
            }
        }
        if let (Some(f), Some(c), Some(ty)) = (func, code, typ) {
            let x = f
                .indexies
                .into_iter()
                .zip(c.codes.into_iter())
                .map(|(f_index, code)| (ty.funcs.get(f_index as usize).unwrap(), code))
                .map(|(f, c)| instance::FunctionInstance::new(f.clone(), c));
        }

        Executor {
            store: Store {},
            stack: Vec::new(),
        }
    }
    fn invoke(&self) {}
}

struct Param {
    func_name: String,
    args: Vec<u8>,
}
