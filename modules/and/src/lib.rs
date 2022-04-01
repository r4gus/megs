extern "C" {    
    fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, r: f32, g: f32, b: f32);    
}    
    
#[no_mangle]    
pub extern "C" fn add_one(x: i32) -> i32 {    
    x + 1    
}    
    
#[no_mangle]    
pub extern "C" fn draw(x: f32, y: f32, r: f32) {    
    unsafe {    
        draw_rectangle(x, y, 100.0, 50.0, 1.0, 0.0, 1.0);    
    }    
}
