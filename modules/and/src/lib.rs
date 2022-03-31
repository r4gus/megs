extern "C" {    
    fn draw_black_rectangle(x: f32, y: f32, w: f32, h: f32);    
}    
    
#[no_mangle]    
pub extern "C" fn add_one(x: i32) -> i32 {    
    x + 1    
}    
    
#[no_mangle]    
pub extern "C" fn draw(x: f32, y: f32, r: f32) {    
    unsafe {    
        draw_black_rectangle(x, y, 100.0, 50.0);    
    }    
}
