extern "C" {    
    fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, r: f32, g: f32, b: f32);    
    fn draw_circle(x: f32, y: f32, rad: f32, r: f32, g: f32, b: f32);
    fn draw_circle_lines(x: f32, y: f32, rad: f32, thick: f32, r: f32, g: f32, b: f32);
    fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thick: f32, r: f32, g: f32, b: f32);
}    

pub const H1: f32 = 10.0;
pub const WIDTH: f32 = H1 * 8.0;
pub const HEIGHT: f32 = H1 * 6.5;

#[no_mangle]
pub extern "C" fn width() -> f32 {
    WIDTH
}

#[no_mangle]
pub extern "C" fn height() -> f32 {
    HEIGHT
}
    
#[no_mangle]    
pub extern "C" fn draw(x: f32, y: f32, r: f32) {    
    unsafe {
        draw_rectangle(x, y, 4.75 * H1, 6.5 * H1, 0.0, 0.0, 0.0);
        draw_circle(x + 4.75 * H1, y + 3.25 * H1, 3.25 * H1, 0.0, 0.0, 0.0);
        draw_rectangle(x + 0.5 * H1, y + 0.5 * H1, 3.75 * H1, 5.5 * H1, 1.0, 1.0, 1.0);
        draw_circle(x + 4.75 * H1, y + 3.25 * H1, 2.75 * H1, 1.0, 1.0, 1.0);
    }    
}
