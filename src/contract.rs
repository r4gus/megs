use macroquad::{shapes, color};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    Undefined,
    Low,
    High,
}

#[repr(C)]
pub struct Color {
    /// red
    pub r: f32,
    /// green
    pub g: f32,
    /// blue
    pub b: f32,
    /// alpha - opaque [0, 1] solid
    pub a: f32,
}

pub fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, r: f32, g: f32, b: f32) {
    shapes::draw_rectangle(x, y, w, h, color::Color::new(r, g, b, 1.));
}

pub fn draw_circle(x: f32, y: f32, rad: f32, r: f32, g: f32, b: f32) {
    shapes::draw_circle(x, y, rad, color::Color::new(r, g, b, 1.));
}

pub fn draw_circle_lines(x: f32, y: f32, rad: f32, thick: f32, r: f32, g: f32, b: f32) {
    shapes::draw_circle_lines(x, y, rad, thick, color::Color::new(r, g, b, 1.));
}

pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thick: f32, r: f32, g: f32, b: f32) {
    shapes::draw_line(x1, y1, x2, y2, thick, color::Color::new(r, g, b, 1.));
}

pub fn draw_arc(x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32, thickness: f32) {

}
