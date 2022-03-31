use wasmer::{Store, Module, Instance, Value, imports, ImportObject, Function};
use macroquad::prelude::*;
use uuid::Uuid;
use crate::misc::{Point};

/// The instance of a [`LogicModule`].
///
/// This class acts as a wrapper around a WebAssembly module
/// to add further functionality like drag'n drop.
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
    instance: Instance,
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
}

/// Represents a WebAssembly module with additional infromation.
pub struct LogicModule {
    /// The name of the component the module represents.
    name: String,
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
    ) -> Option<LogicInstance> {
        if let Ok(instance) = Instance::new(&self.module, imports) {
            Some(LogicInstance::new(self.name.clone(), location, rotation, instance))
        } else {
            None
        }
    }
}

/// A [`Category`] groups a number of [`LogicModules`].
pub struct Category {
    /// The name of the group.
    name: String,
    /// A unique group-id.
    id: usize,
    /// A list of [`LogicModules`].
    modules: Vec<LogicModule>,
}

impl Category {
    /// Create a new category.
    pub fn new(name: String, id: usize) -> Self {
        Self {
            name,
            id,
            modules: Vec::new(),
        }
    }
    
    /// Add a [`LogicModule`] to the category.
    pub fn add_module(&mut self, module: LogicModule) {
        self.modules.push(module);
    }
    
    /// Get the [`LogicModules`] the given category contains.
    pub fn modules(&self) -> &Vec<LogicModule> {
        &self.modules
    }
}

pub struct ModuleEnv {
    /// The store represents all global state that can be
    /// manipulated by WebAssembly programs. It consists
    /// of the runtime representation of all instances of
    /// functions, tables, memories, and globals that have
    /// been allocated during the lifetime of the abstract
    /// machine [`https://docs.rs/wasmer/latest/wasmer/struct.Store.html`].
    store: Store,
    /// A list of existing categories. Each category contains a set of modules.
    categories: Vec<Category>,
    /// All instances of [`LogicModules`].
    instances: Vec<LogicInstance>,
    /// A import contract all modules should obey.
    imports: ImportObject,
    /// Global category counter.
    cat_id: usize,
    /// Global module counter
    mod_id: usize,
}

impl ModuleEnv {
    pub fn new() -> Self {
        let store = Store::default();

        Self {
            store: store.clone(),
            categories: Vec::new(),
            instances: Vec::new(),
            imports: imports! {
                "megs" => {
                    "draw_black_rectangle" => Function::new_native(&store, draw_black_rectangle),
                },
            },
            cat_id: 0,
            mod_id: 0,
        }
    }

    pub fn categories(&self) -> &Vec<Category> {
        &self.categories
    }

    pub fn instances(&self) -> &Vec<LogicInstance> {
        &self.instances
    }

    pub fn on_tick(&self) {
        for instance in &self.instances {
            instance.draw();
        }
    }

    pub fn add_category(&mut self, name: String) {
        self.categories.push(
            Category::new(name, self.cat_id)
        );
        self.cat_id += 1;
    }
    
    pub fn add_module(&mut self, category: usize, name: String, module: &[u8]) -> Result<(), ()> {
        if category >= self.categories.len() {
            return Err(());
        }

        if let Ok(module) = Module::new(&self.store, module) {

            self.categories[category].add_module(
                LogicModule::new(
                    name,
                    self.mod_id,
                    module,
                )
            );

            self.mod_id += 1;

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn instantiate(&mut self, category: usize, module: usize, pos: Point) -> Option<Uuid> {
        if category >= self.categories.len() || 
            module >= self.categories[category].modules().len() {
            return None;
        }

        if let Some(instance) = self.categories[category].modules()[module].instantiate(
            &self.imports, pos, 0.0
        ) {
            let uuid = Some(instance.id());
            self.instances.push(instance);
            uuid
        } else {
            None
        }
    }
}

fn draw_black_rectangle(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0., 0., 0., 1.));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_module_env_test() {
        let env = ModuleEnv::new();
        assert_eq!(0, env.categories().len());
        assert_eq!(0, env.instances().len());
    }

    #[test]
    fn add_category_test() {
        let mut env = ModuleEnv::new();
        env.add_category("Gates".to_string());
        env.add_category("Input Controlls".to_string());
        assert_eq!(2, env.categories().len());
        assert_eq!(0, env.instances().len());
    }

    #[test]
    fn add_modules_test() {
        let module_wat = r#"
            (module
                (import "megs" "draw_black_rectangle" (func $dbr (param f32 f32 f32 f32)))
                (func $draw (export "draw") 
                    (param $x f32) (param $y f32) (param $r f32)
                    
                    (call $dbr (local.get $x) (local.get $y) (f32.const 100.0) (f32.const 50.0))
                )
            )
        "#;

        let mut env = ModuleEnv::new();
        env.add_category("Gates".to_string());
        env.add_category("Input Controlls".to_string());
        env.add_module(0, "AND".to_string(), module_wat.as_bytes());
        assert_eq!(1, env.categories()[0].modules().len());
        assert_eq!(0, env.categories()[1].modules().len());

        env.instantiate(0, 0, Point { x: 0.0, y: 0.0 });
        env.instantiate(0, 0, Point { x: 50.0, y: 30.0 });
        env.instantiate(0, 0, Point { x: -15.0, y: 200.0 });
        assert_eq!(3, env.instances().len());
    }
}
