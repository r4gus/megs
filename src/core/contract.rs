use wasmer::{ExportType, ImportType, ImportObject, Module, imports, ExternType};
use std::error::Error;
use std::fmt;

/// A [`Contract`] describes what the environment expects
/// from a given module and what the module can expect
/// in return.
pub struct Contract {
    /// A set of exports that a module must provide.
    pub exports: Vec<ExportType>,
    /// A set of imports a module can expect.
    pub imports: Vec<ImportType>,
}

impl Contract {
    fn format_export(exp: &ExportType) -> String {
        let mut s = format!("{}(param", exp.name());

        match exp.ty() {
            ExternType::Function(ft) => {
                for p in ft.params().iter() {
                    s += &format!(" {:?}", p);
                }
                s += ")(result";
                for r in ft.results().iter() {
                    s += &format!(" {:?}", r); 
                }
                s += ")";
            },
            ExternType::Global(gt) => {

            },
            ExternType::Table(tt) => {

            },
            ExternType::Memory(mt) => {

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
                format!("missing export `{}`", Contract::format_export(export))
            ));
        }

        'imp: for import in self.imports.iter() {
            for import_ in module.imports() {
                if *import == import_ {
                    // Found the required import.
                    continue 'imp;
                }
            }

            // Seems the given import is missing.
            // NOTE: This isn't that bad but we keep
            // it conservative.
            return Err(ContractError::ImportErr(
                format!("missing import `{:?}`", import)
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

#[cfg(test)]
mod tests {
    use super::*;
    use wasmer::{ExportType, ImportType, FunctionType, ImportObject, Module, imports, Store, Type, ExternType};
    
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
                    "missing export `width(param)(result F32)`".to_string())), 
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
                    "missing export `draw(param F32 F32)(result)`".to_string())), 
            contract.check(&module)
        );
    }
}














