use wasmer::{Store, Module, Instance, Value, imports, ImportObject, FunctionType, Type, ImportType, ExternType, Function, ExportType};
use macroquad::prelude::*;
use uuid::Uuid;
use std::{
    io::{Read},
    path::{Path},
    fs::{File},
    collections::HashMap,
    convert::From,
    error::Error,
    fmt,
};
use crate::misc::{Point, parse_path};
use crate::core::contract::*;

#[derive(Debug)]
pub enum ModuleError {
    CompileErr(wasmer::CompileError),
    ContractErr(ContractError),
    IOErr(std::io::Error),
}

impl From<wasmer::CompileError> for ModuleError {
    fn from(e: wasmer::CompileError) -> Self {
        Self::CompileErr(e)
    }
}

impl From<ContractError> for ModuleError {
    fn from(e: ContractError) -> Self {
        Self::ContractErr(e)
    }
}

impl From<std::io::Error> for ModuleError {
    fn from(e: std::io::Error) -> Self {
        Self::IOErr(e)
    }
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleError::CompileErr(e) => {
                write!(f, "{}", &e)
            },
            ModuleError::ContractErr(e) => {
                write!(f, "{}", &e)
            },
            ModuleError::IOErr(e) => {
                write!(f, "{}", &e)
            },
        }
    }
}

impl Error for ModuleError {

}

/// The instance of a [`LogicModule`].
///
/// This class acts as a wrapper around a WebAssembly module
/// to add further functionality like drag'n drop.
#[derive(Debug, Clone)]
pub struct LogicInstance {
    /// The name of the instance (e.g. 'AND', 'My custom gate', ...).
    pub name: String,
    /// A unique identifier.
    id: Uuid,
    /// The position of the instance in 2d space.
    pub location: Point,
    /// The rotation of the instance in deg.
    pub rotation: f32,
    /// The WebAssembly instance that contains the actual logic.
    pub instance: Instance,
}

impl LogicInstance {
    /// Create a new instance.
    fn new(name: String, location: Point, rotation: f32, instance: Instance) -> Self {
        Self {
            name,
            id: Uuid::new_v4(),
            location,
            rotation,
            instance,
        }
    }
    
    /// Get the Uuid of the instance.
    pub fn id(&self) -> Uuid {
        self.id.clone()
    }
    
    /// Draw the instance
    ///
    /// TODO: Encapsulate specific function within trait???
    pub fn draw(&self) {
        if let Ok(draw) = self.instance.exports.get_function("draw") {
            draw.call(&[Value::F32(self.location.x), Value::F32(self.location.y), Value::F32(self.rotation)]);
        } else {
            // TODO: log error
        }
    }

    pub fn submit_cursor_coords(&self, point: Point) {
        if let Ok(submit) = self.instance.exports.get_function("cursor_coords") {
            submit.call(&[Value::F32(point.x), Value::F32(point.y)]);
        } else {
            // TODO: log error
        }
    }
}

/// Represents a WebAssembly module with additional infromation.
#[derive(Debug, Clone)]
pub struct LogicModule {
    /// The name of the component the module represents.
    pub name: String,
    /// A unique id.
    id: usize,
    /// A range of valid inputs.
    inputs: (usize, usize),
    /// A range of valid outputs.
    outputs: (usize, usize),
    /// The actual WebAssembly module.
    module: Module,
}

impl LogicModule {
    /// Create a new [`LogicModule`].
    pub fn new(name: String, id: usize, module: Module) -> Self {
        Self {
            name,
            id,
            inputs: (2, 2),
            outputs: (1, 1),
            module,
        }
    }
    
    /// Get a reference to the WebAssembly module.
    pub fn module(&self) -> &Module {
        &self.module
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> usize {
        self.id
    }
    
    /// Create a new instance based on the given module.
    pub fn instantiate(
        &self, 
        imports: &ImportObject, 
        location: Point, 
        rotation: f32
    ) -> Result<LogicInstance, wasmer::InstantiationError> {
        Ok(
            LogicInstance::new(
                self.name.clone(), 
                location, 
                rotation, 
                Instance::new(&self.module, imports)?
            )
        )
    }
}

/// A [`Category`] groups a number of [`LogicModules`].
#[derive(Debug, Clone)]
pub struct Category {
    /// The name of the group.
    name: String,
    /// A unique group-id.
    id: usize,
    /// A list of [`LogicModules`].
    modules: HashMap<String, LogicModule>,
}

impl Category {
    /// Create a new category.
    pub fn new(name: String, id: usize) -> Self {
        Self {
            name,
            id,
            modules: HashMap::new(),
        }
    }
    
    /// Add a [`LogicModule`] to the category.
    pub fn add_module(&mut self, module: LogicModule) {
        self.modules.insert(module.name.clone(), module);
    }
    
    /// Get the [`LogicModules`] the given category contains.
    pub fn modules(&self) -> &HashMap<String, LogicModule> {
        &self.modules
    }
}

#[derive(Debug, Clone)]
pub struct ModuleEnv {
    /// The store represents all global state that can be
    /// manipulated by WebAssembly programs. It consists
    /// of the runtime representation of all instances of
    /// functions, tables, memories, and globals that have
    /// been allocated during the lifetime of the abstract
    /// machine [`https://docs.rs/wasmer/latest/wasmer/struct.Store.html`].
    store: Store,
    /// A list of existing categories. Each category contains a set of modules.
    categories: HashMap<String, Category>,
    /// All instances of [`LogicModules`].
    instances: HashMap<Uuid, LogicInstance>,
    /// A import contract all modules should obey.
    imports: ImportObject,
    /// A contract that all module must obey.
    contract: Contract,
    /// Global category counter.
    cat_id: usize,
    /// Global module counter
    mod_id: usize,
}

impl ModuleEnv {
    /// Create a new [`ModuleEnv`] using the given store and contract.
    ///
    /// The store (see [`wasmer::Store`]) represents the global state
    /// of the environment and the contract specifies which globals,
    /// functions, ... all modules expect from the host environment
    /// as imports.
    pub fn new(store: Store, imports: ImportObject, contract: Contract) -> Self {
        Self {
            store: store.clone(),
            categories: HashMap::new(),
            instances: HashMap::new(),
            imports,
            contract,
            cat_id: 0,
            mod_id: 0,
        }
    }
    
    /// Get a reference to all existing categories.
    pub fn categories(&self) -> &HashMap<String, Category> {
        &self.categories
    }
    
    /// Get the names of all available categories.
    pub fn category_names(&self) -> Vec<&String> {
        self.categories.keys().collect()
    }
    
    /// Get all module names of the given category.
    pub fn module_names(&self, category: &str) -> Option<Vec<&String>> {
        match self.categories.get(category) {
            Some(category) => {
                Some(category.modules().keys().collect())
            },
            None => None
        }
    }
    
    /// Get a reference to all existing instances.
    pub fn instances(&self) -> &HashMap<Uuid, LogicInstance> {
        &self.instances
    }
    
    pub fn on_tick(&self) {
        for (_, instance) in &self.instances {
            instance.draw();
        }
    }
    
    /// Add a new category with the given name to the environment.
    pub fn add_category(&mut self, name: String) {
        self.categories.insert(
            name.clone(),
            Category::new(name, self.cat_id)
        );
        self.cat_id += 1;
    }
    
    /// Add a new module to the specified category.
    ///
    /// If the category doesn't exist, a new one is created.
    ///
    /// After adding the module one can create new instances of
    /// it by invoking [`ModuleEnv::instantiate`].
    pub fn add_module_raw(
        &mut self, 
        category: &str, 
        name: &str, 
        module: &[u8],
    ) -> Result<(), ModuleError> {
        // Create the category if it doesn't exist.
        if !self.categories.contains_key(category) {
            self.add_category(category.to_string());
        }
        
        let id = self.mod_id;

        let module = Module::new(&self.store, module)?;
        self.contract.check(&module)?;

        self.categories.get_mut(category).unwrap().add_module(
            LogicModule::new(
                name.to_string(),
                id,
                module,
            )
        );

        self.mod_id += 1;
        Ok(())
    }
    
    /// Add WebAssembly module from file path to the specified category.
    ///
    /// If the category doesn't exist, a new one is created.
    ///
    /// After adding the module one can create new instances of
    /// it by invoking [`ModuleEnv::instantiate`].
    pub fn add_module(&mut self, wasm_file: &Path) -> Result<(), ModuleError> {
        let mut buffer = Vec::new();
        let mut module = File::open(wasm_file)?;
        module.read_to_end(&mut buffer)?;

        let (category, name) = parse_path(wasm_file).expect("invalid path");
        println!("{}, {}", &category, &name);
        self.add_module_raw(&category, &name, &buffer)
    }
    
    /// Create a new instance of the specified module.
    pub fn instantiate(&mut self, category: &str, module: &str, pos: Point) -> Option<Uuid> {
        if !self.categories.contains_key(category) || 
            !self.categories[category].modules().contains_key(module) {
            return None;
        }

        if let Ok(instance) = self.categories[category].modules()[module].instantiate(
            &self.imports, pos, 0.0
        ) {
            let uuid = Some(instance.id());
            self.instances.insert(instance.id(), instance);
            uuid
        } else {
            println!("fuck");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, r: f32, g: f32, b: f32) {

    }

    #[test]
    fn create_new_module_env_test() {
        let store = Store::default();
        let imports = imports! {
            "env" => {
                "draw_rectangle" => Function::new_native(&store, draw_rectangle),
            },
        };
        let contract = Contract {
            exports: vec![],
            imports: vec![
                ImportType::new("env", "draw_rectangle", ExternType::Function(FunctionType::new([Type::F32, Type::F32, Type::F32, Type::F32, Type::F32, Type::F32, Type::F32], []))),
            ],
        };

        let env = ModuleEnv::new(store, imports, contract);
        assert_eq!(0, env.categories().len());
        assert_eq!(0, env.instances().len());
    }

    #[test]
    fn add_category_test() {
        let store = Store::default();
        let imports = imports! {
            "env" => {
                "draw_rectangle" => Function::new_native(&store, draw_rectangle),
            },
        };
        let contract = Contract {
            exports: vec![],
            imports: vec![
                ImportType::new("env", "draw_rectangle", ExternType::Function(FunctionType::new([Type::F32, Type::F32, Type::F32, Type::F32, Type::F32, Type::F32, Type::F32], []))),
            ],
        };
        let mut env = ModuleEnv::new(store, imports, contract);
        env.add_category("Gates".to_string());
        env.add_category("Input Controlls".to_string());
        assert_eq!(2, env.categories().len());
        assert_eq!(0, env.instances().len());

        let mut names = env.category_names();
        names.sort();
        assert_eq!("Gates", names[0]);
        assert_eq!("Input Controlls", names[1]);
    }

    #[test]
    fn add_modules_raw_test() {
        let module_wat = r#"
            (module
                (import "env" "draw_rectangle" (func $dbr (param f32 f32 f32 f32 f32 f32 f32)))
                (func $draw (export "draw") 
                    (param $x f32) (param $y f32) (param $r f32)
                    
                    (call $dbr (local.get $x) (local.get $y) (f32.const 100.0) (f32.const 50.0) (f32.const 1.0) (f32.const 1.0) (f32.const 1.0))
                )
            )
        "#;

        let store = Store::default();
        let imports = imports! {
            "env" => {
                "draw_rectangle" => Function::new_native(&store, draw_rectangle),
            },
        };
        let contract = Contract {
            exports: vec![
                ExportType::new("draw", ExternType::Function(FunctionType::new([Type::F32, Type::F32, Type::F32], []))),
            ],
            imports: vec![
                ImportType::new("env", "draw_rectangle", ExternType::Function(FunctionType::new([Type::F32, Type::F32, Type::F32, Type::F32, Type::F32, Type::F32, Type::F32], []))),
            ],
        };
        let mut env = ModuleEnv::new(store, imports, contract);
        env.add_category("Gates".to_string());
        env.add_category("Input Controlls".to_string());
        env.add_module_raw("Gates", "AND", module_wat.as_bytes());
        assert_eq!(1, env.categories()["Gates"].modules().len());
        assert_eq!(0, env.categories()["Input Controlls"].modules().len());
        assert_eq!("AND", env.module_names("Gates").unwrap()[0]);

        env.instantiate("Gates", "AND", Point { x: 0.0, y: 0.0 });
        env.instantiate("Gates", "AND", Point { x: 50.0, y: 30.0 });
        env.instantiate("Gates", "AND", Point { x: -15.0, y: 200.0 });
        assert_eq!(3, env.instances().len());
    }
}
