extern "C" {    
    fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, r: f32, g: f32, b: f32);    
    fn draw_circle(x: f32, y: f32, rad: f32, r: f32, g: f32, b: f32);
    fn draw_circle_lines(x: f32, y: f32, rad: f32, thick: f32, r: f32, g: f32, b: f32);
    fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thick: f32, r: f32, g: f32, b: f32);
}    

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    Undefined,
    Low,
    High,
}

pub const H1: f32 = 10.0;
pub const WIDTH: f32 = H1 * 8.0;
pub const HEIGHT: f32 = H1 * 6.5;

pub const INPUTS_MIN: i32 = 2;
pub const INPUTS_MAX: i32 = 8;
static mut INPUTS_CUR: i32 = INPUTS_MIN;
static mut INPUTS: [State; INPUTS_MAX as usize] = [State::Undefined; INPUTS_MAX as usize];

pub const OUTPUTS_MIN: i32 = 1;
pub const OUTPUTS_MAX: i32 = 1;
static mut OUTPUTS_CUR: i32 = OUTPUTS_MIN;
static mut OUTPUTS: [State; OUTPUTS_MAX as usize] = [State::Undefined; OUTPUTS_MAX as usize];

#[no_mangle]
pub extern "C" fn get_inputs_nr() -> i32 {
    unsafe {
        return INPUTS_CUR;
    }
}

#[no_mangle]
pub extern "C" fn set_inputs_nr(v: i32) {
    if v >= INPUTS_MIN && v <= INPUTS_MAX {
        unsafe {
            INPUTS_CUR = v;
        }
    }
}

#[no_mangle]
pub extern "C" fn get_input(i: i32) -> Option<State> {
    unsafe {
        if i < INPUTS_CUR {
            return  Some(INPUTS[i as usize]);
        }
    }
    None
}

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

