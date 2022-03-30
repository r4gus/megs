use wasmer::{Store, Module, Instance, Value, imports, ImportObject, Function};
use macroquad::prelude::*;
use uuid::Uuid;
use crate::misc::{Point};

pub struct LogicInstance {
    name: String,
    id: Uuid,
    location: Point,
    rotation: f32,
    instance: Instance,
}

impl LogicInstance {
    pub fn new(name: String, location: Point, instance: Instance) -> Self {
        Self {
            name,
            id: Uuid::new_v4(),
            location,
            rotation: 0.0,
            instance,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn draw(&self) {
        if let Ok(draw) = self.instance.exports.get_function("draw") {
            draw.call(&[Value::F32(self.location.x), Value::F32(self.location.y), Value::F32(self.rotation)]);
        } else {
            // TODO: log error
        }
    }
}

pub struct LogicModule {
    name: String,
    id: usize,
    inputs: (usize, usize),
    outputs: (usize, usize),
    module: Module,
}

impl LogicModule {
    pub fn new(name: String, id: usize, module: Module) -> Self {
        Self {
            name,
            id,
            inputs: (2, 2),
            outputs: (1, 1),
            module,
        }
    }

    pub fn module(&self) -> &Module {
        &self.module
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

pub struct Category {
    name: String,
    id: usize,
    modules: Vec<LogicModule>,
}

impl Category {
    pub fn new(name: String, id: usize) -> Self {
        Self {
            name,
            id,
            modules: Vec::new(),
        }
    }

    pub fn add_module(&mut self, module: LogicModule) {
        self.modules.push(module);
    }

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
    categories: Vec<Category>,
    instances: Vec<LogicInstance>,
    imports: ImportObject,
    cat_id: usize,
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

    pub fn instantiate(&mut self, category: usize, module: usize, pos: Point) -> Result<Uuid, ()> {
        if category >= self.categories.len() || 
            module >= self.categories[category].modules().len() {
            return Err(());
        }

        if let Ok(instance) = Instance::new(
            &self.categories[category].modules()[module].module(),
            &self.imports
        ) {

            let li = LogicInstance::new(
                self.categories[category].modules()[module].name().to_string(),
                pos,
                instance,
            );
            let uuid = li.id();
            self.instances.push(li);
            Ok(uuid) 
        } else {
            Err(())
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
