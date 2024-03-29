use macroquad::prelude::*;
use macroquad::ui::root_ui;
use megs::core::module::*;
use megs::misc::*;
use megs::core::contract::*;
use wasmer::{Store, Function, imports, Type, ImportType, ExportType, FunctionType, ExternType};

#[macroquad::main("MEGS")]
async fn main() {
    let module_wat = r#"
        (module
            (import "megs" "draw_black_rectangle" (func $dbr (param f32 f32 f32 f32)))
            (func $draw (export "draw") 
                (param $x f32) (param $y f32) (param $r f32)
                
                (call $dbr (local.get $x) (local.get $y) (f32.const 100.0) (f32.const 50.0))
            )
        )
    "#;
    
    let mut store = Store::default();
    let imports = imports! {
        "env" => {
            "draw_rectangle" => Function::new_native(&mut store, megs::contract::draw_rectangle),
            "draw_circle" => Function::new_native(&mut store, megs::contract::draw_circle),
            "draw_circle_lines" => Function::new_native(&mut store, megs::contract::draw_circle_lines),
            "draw_line" => Function::new_native(&mut store, megs::contract::draw_line),
        },
    };
    let contract = Contract {
        exports: vec![
            ExportType::new("width", ExternType::Function(FunctionType::new([], [Type::F32]))),
            ExportType::new("height", ExternType::Function(FunctionType::new([], [Type::F32]))),
            ExportType::new("draw", ExternType::Function(FunctionType::new([Type::F32, Type::F32, Type::F32], []))),
        ],
        imports: inobj_types(&imports, &store),
    };

    let mut env = ModuleEnv::new(store, imports, contract);
    env.add_category("Gates".to_string());
    //env.add_module_raw("Gates", "AND", module_wat.as_bytes());
    match env.add_module(&std::path::Path::new("assets/modules/Gates/and.wasm")) {
        Err(e) => println!("{}", e),
        _ => {}
    };
    env.instantiate("Gates", "and", Point { x: 0.0, y: 0.0 });
    env.instantiate("Gates", "and", Point { x: 50.0, y: 30.0 });
    env.instantiate("Gates", "and", Point { x: -15.0, y: 200.0 });


    println!("{}", env.categories()["Gates"].modules().len());
    println!("{}", env.instances().len());

    loop {
        clear_background(RED);

        env.on_tick();
        
        /*
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);


        if root_ui().button(None, "Push me") {
           println!("pushed");
        }
        */
        

        draw_text(&format!("fps: {}", get_fps()), 300.0, 500.0, 30.0, BLACK);

        next_frame().await
    }
}
