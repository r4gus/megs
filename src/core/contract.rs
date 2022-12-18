use wasmer::{ExportType, ImportType, Imports, Module, imports, ExternType, AsStoreRef};
use std::{
    error::Error,
    fmt,
    convert::From,
};

/// A [`Contract`] describes what the environment expects
/// from a given module and what the module can expect
/// in return.
#[derive(Debug, Clone)]
pub struct Contract {
    /// A set of exports that a module must provide.
    pub exports: Vec<ExportType>,
    /// A set of imports a module can expect.
    pub imports: Vec<ImportType>,
}

impl Contract {
    fn format_extern(ext: &ExternType) -> String {
        let mut s = String::new();

        match ext {
            ExternType::Function(ft) => {
                s += "(param";
                for p in ft.params().iter() {
                    s += &format!(" {:?}", p);
                }
                s += ")(result";
                for r in ft.results().iter() {
                    s += &format!(" {:?}", r); 
                }
                s += ") [function]";
            },
            ExternType::Global(gt) => {
                s += &format!(" {:?} {:?} [global]", gt.ty, gt.mutability);
            },
            ExternType::Table(tt) => {
                s += " [table]";
            },
            ExternType::Memory(mt) => {
                s += " [memory]";
            }
        }

        s
    }

    pub fn check(&self, module: &Module) -> Result<(), ContractError> {
        // First verify that all exports are satisfied.
        'exp: for export in self.exports.iter() {
            for export_ in module.exports() {
                if *export == export_ {
                    // Found the required export.
                    continue 'exp;
                }
            }

            // Seems to be missing.
            return Err(ContractError::ExportErr(
                format!("missing export `{}{}`", export.name(), Contract::format_extern(export.ty()))
            ));
        }

        'imp: for import in module.imports() {
            for import_ in self.imports.iter() {
                if import == *import_ {
                    // Found the required import.
                    continue 'imp;
                }
            }

            return Err(ContractError::ImportErr(
                format!("missing import `{}::{}{}`", import.module(), import.name(), Contract::format_extern(import.ty()))
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContractError {
    ExportErr(String),
    ImportErr(String),
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractError::ExportErr(e) => {
                write!(f, "{}", &e)
            },
            ContractError::ImportErr(e) => {
                write!(f, "{}", &e)
            }
        }
    }
}

impl Error for ContractError {
    
}

/// Translate the exports of the given [`Imports`] into
/// [`ImportType`]s.
pub fn inobj_types(io: &Imports, store: &impl AsStoreRef) -> Vec<ImportType> {
    let mut v = Vec::new();

    for ((module, name), ext) in io.clone().into_iter() {
        v.push(
            ImportType::new(
                &module, 
                &name, 
                ext.ty(store)
            )
        );
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasmer::{ExportType, ImportType, FunctionType, Imports, Module, imports, Store, Type, ExternType};
    
    #[test]
    fn check_contract_test() {
        let module_wat = r#"
            (module
                (import "env" "draw_rectangle" (func $dr (param f32 f32 f32 f32)))
                (func (export "width") (result f32)
                    f32.const 80
                )
                (func (export "draw") (param $x f32) (param $y f32)
                    (call $dr (local.get $x) (local.get $y) (f32.const 80.0) (f32.const 60.0))
                )
            )
        "#;
        
        let store = Store::default();
        let module = Module::new(&store, module_wat).expect("unable to create module");
        
        let contract = Contract {
            exports: vec![
                ExportType::new("width", ExternType::Function(FunctionType::new([], [Type::F32]))),
                ExportType::new("draw", ExternType::Function(FunctionType::new([Type::F32, Type::F32], []))),
            ],
            imports: vec![
                ImportType::new("env", "draw_rectangle", ExternType::Function(FunctionType::new([Type::F32, Type::F32, Type::F32, Type::F32], []))), 
            ],
        };

        assert_eq!(Ok(()), contract.check(&module));
    }
    
    #[test]
    fn missing_export_test() {
        let module_wat = r#"
            (module
                (import "env" "draw_rectangle" (func $dr (param f32 f32 f32 f32)))
                (func (export "draw") (param $x f32) (param $y f32)
                    (call $dr (local.get $x) (local.get $y) (f32.const 80.0) (f32.const 60.0))
                )
            )
        "#;
        
        let store = Store::default();
        let module = Module::new(&store, module_wat).expect("unable to create module");
        
        let contract = Contract {
            exports: vec![
                ExportType::new("width", ExternType::Function(FunctionType::new([], [Type::F32]))),
                ExportType::new("draw", ExternType::Function(FunctionType::new([Type::F32, Type::F32], []))),
            ],
            imports: vec![
                ImportType::new("env", "draw_rectangle", ExternType::Function(FunctionType::new([Type::F32, Type::F32, Type::F32, Type::F32], []))), 
            ],
        };

        assert_eq!(
            Err(ContractError::ExportErr(
                    "missing export `width(param)(result F32) [function]`".to_string())), 
            contract.check(&module)
        );
    }

    #[test]
    fn missing_export_test2() {
        let module_wat = r#"
            (module
                (import "env" "draw_rectangle" (func $dr (param f32 f32 f32 f32)))
                (func (export "width") (result f32)
                    f32.const 80
                )
            )
        "#;
        
        let store = Store::default();
        let module = Module::new(&store, module_wat).expect("unable to create module");
        
        let contract = Contract {
            exports: vec![
                ExportType::new("width", ExternType::Function(FunctionType::new([], [Type::F32]))),
                ExportType::new("draw", ExternType::Function(FunctionType::new([Type::F32, Type::F32], []))),
            ],
            imports: vec![
                ImportType::new("env", "draw_rectangle", ExternType::Function(FunctionType::new([Type::F32, Type::F32, Type::F32, Type::F32], []))), 
            ],
        };

        assert_eq!(
            Err(ContractError::ExportErr(
                    "missing export `draw(param F32 F32)(result) [function]`".to_string())), 
            contract.check(&module)
        );
    }

    #[test]
    fn missing_import_test() {
        let module_wat = r#"
            (module
                (import "env" "draw_rectangle" (func $dr (param f32 f32 f32 f32)))
                (func (export "width") (result f32)
                    f32.const 80
                )
                (func (export "draw") (param $x f32) (param $y f32)
                )
            )
        "#;
        
        let store = Store::default();
        let module = Module::new(&store, module_wat).expect("unable to create module");
        
        let contract = Contract {
            exports: vec![
                ExportType::new("width", ExternType::Function(FunctionType::new([], [Type::F32]))),
                ExportType::new("draw", ExternType::Function(FunctionType::new([Type::F32, Type::F32], []))),
            ],
            imports: vec![
            ],
        };

        assert_eq!(
            Err(ContractError::ImportErr(
                    "missing import `env::draw_rectangle(param F32 F32 F32 F32)(result) [function]`".to_string())), 
            contract.check(&module)
        );
    }

    #[test]
    fn from_import_object() {
        let module_wat = r#"
            (module
                (import "env" "draw_rectangle" (func $dr (param f32 f32 f32 f32)))
                (func (export "width") (result f32)
                    f32.const 80
                )
                (func (export "draw") (param $x f32) (param $y f32)
                    (call $dr (local.get $x) (local.get $y) (f32.const 80.0) (f32.const 60.0))
                )
            )
        "#;
        
        // dummy
        fn draw_rectangle(x: f32, y: f32, w: f32, h: f32) {

        }
        
        let mut store = Store::default();
        let imports = imports! {
            "env" => {
                "draw_rectangle" => wasmer::Function::new_native(&mut store, draw_rectangle),
            },
        };
        let module = Module::new(&store, module_wat).expect("unable to create module");
        
        let contract = Contract {
            exports: vec![
                ExportType::new("width", ExternType::Function(FunctionType::new([], [Type::F32]))),
                ExportType::new("draw", ExternType::Function(FunctionType::new([Type::F32, Type::F32], []))),
            ],
            imports: inobj_types(&imports, &store),
        };

        assert_eq!(Ok(()), contract.check(&module));
    }
}














