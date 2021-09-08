use std::{collections::HashMap, hash::Hash};

use crate::{
    ast::{module, section},
    object::{self, instance, value},
};
struct Store {
    funcs: Vec<instance::FunctionInstance>,
}
struct Executor {
    store: Store,
    stack: Vec<u128>,
    export_map: HashMap<String, object::value::ExternVal>,
}

impl Executor {
    fn new(module: module::Module) -> Self {
        let mut func = None;
        let mut code = None;
        let mut typ = None;
        let mut exp = HashMap::new();
        for s in module.sections {
            match s.payload_data {
                crate::ast::section::SectionData::Custom(_) => {} // do noting
                crate::ast::section::SectionData::Type(t) => typ = Some(t),
                crate::ast::section::SectionData::Import => {}
                crate::ast::section::SectionData::Function(f) => func = Some(f),
                crate::ast::section::SectionData::Table => {}
                crate::ast::section::SectionData::Memory => {}
                crate::ast::section::SectionData::Global => {}
                crate::ast::section::SectionData::Export(ex) => {
                    for e in ex.exports {
                        if let section::ExportDesc::FuncIndex(index) = e.desc {
                            exp.insert(
                                String::from_utf8(e.name).unwrap(),
                                object::value::ExternVal::FuncAddr(index),
                            );
                        }
                    }
                }
                crate::ast::section::SectionData::Start => {}
                crate::ast::section::SectionData::Element => {}
                crate::ast::section::SectionData::Code(c) => code = Some(c),
                crate::ast::section::SectionData::Data => {}
                crate::ast::section::SectionData::DataCount => {}
            }
        }

        let mut funcs = vec![];
        if let (Some(f), Some(c), Some(ty)) = (func, code, typ) {
            funcs = f
                .indexies
                .into_iter()
                .zip(c.codes.into_iter())
                .map(|(f_index, code)| (ty.funcs.get(f_index as usize).unwrap(), code))
                .map(|(f, c)| instance::FunctionInstance::new(f.clone(), c))
                .collect();
        }

        Executor {
            store: Store { funcs },
            stack: Vec::new(),
            export_map: exp,
        }
    }
    fn invoke(&self, param: Parameter) {
        // TODO 戻り値最高
        let extern_val = self.export_map.get(&param.func_name);
        if extern_val.is_some() {
            println!("exist")
        } else {
            println!("no exist")
        }

        let extern_val = extern_val.unwrap(); //FIXME とりあえず

        let value::ExternVal::FuncAddr(index) = extern_val;
        let f = self.store.funcs.get(*index as usize);
        if f.is_some() {
            println!("exist")
        } else {
            println!("no exist")
        }
        // TODO 引数も一bimyou ni致するか確認
    }
}

struct Parameter {
    func_name: String,
}

impl Parameter {
    fn new(func_name: String) -> Self {
        Parameter { func_name }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::{Executor, Parameter};
    #[test]
    fn call() {
        let input: &[u8] = &[
            0x00, 0x61, 0x73, 0x6d, // magic number
            0x01, 0x00, 0x00, 0x00, // version
            // type section
            0x01, 0x07, // id, length
            0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, // a
            // function section
            0x03, 0x02, // id, length
            0x01, 0x00, // a
            // export section
            0x07, 0x07, // id, length
            0x01, 0x03, 0x61, 0x64, 0x64, 0x00, 0x00, // a
            // code section
            0x0a, 0x09, // id, length
            0x01, // number of element
            0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b, // a
            // custom section (ignore)
            0x00, 0x1c, // id, length
            0x04, 0x6e, 0x61, 0x6d, 0x65, 0x01, 0x06, 0x01, 0x00, 0x03, 0x61, 0x64, 0x64, 0x02,
            0x0d, 0x01, 0x00, 0x02, 0x00, 0x03, 0x6c, 0x68, 0x73, 0x01, 0x03, 0x72, 0x68, 0x73,
        ];
        let input = &mut Cursor::new(input);
        let module = crate::ast::module::Module::parse(input).unwrap();
        let exe = Executor::new(module);
        let param = Parameter::new("add".to_string());
        exe.invoke(param);
    }
}
