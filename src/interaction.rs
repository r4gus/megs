use crate::{
    ui::dnd::*,
    core::module::*,
    misc::Point,
};
use uuid::Uuid;
use macroquad::prelude::*;

pub struct InteractionEnv {
    selected: Vec<(Uuid, ClickOffset)>,
    mb_down: bool,
}

impl InteractionEnv {
    pub fn new() -> Self {
        Self {
            selected: Vec::new(),
            mb_down: false,
        }
    }

    pub fn update(&mut self, env: &mut ModuleEnv) {
        if is_mouse_button_down(MouseButton::Left) {
            let mpos = mouse_position();
            let point = Point { x: mpos.0, y: mpos.1, z: 0.0 };

            if self.mb_down {

            } else {
                self.mb_down = true;
                // Left button just pressed.
                for (id, instance) in env.mut_instances() {
                    if let Some(click_offset) = instance.clicked(point) {
                        if self.selected.is_empty() {
                            self.selected.push((*id, click_offset));
                        } else if self.selected.len() > 1 {

                        } else {
                            if self.selected[0].1.z <= click_offset.z {
                                self.selected.remove(0);
                                self.selected.push((*id, click_offset));
                            }
                        }
                    }
                }
            }
        } else {
            self.mb_down = false;
        }
        
        if !self.selected.is_empty() {
            println!("{:?} : {:?}", self.selected[0].0, self.selected[0].1);
        }
    }
}

impl Hoverable for LogicInstance {
    fn hovered(&self, point: Point) -> bool {
        let width = self.width();
        let height = self.height();

        if point.x >= self.location.x && 
           point.x <= self.location.x + width &&
           point.y >= self.location.y &&
           point.y <= self.location.y + height
        {
            true
        } else { 
            false
        }
    }
}

impl Clickable for LogicInstance {
    fn clicked(&mut self, point: Point) -> Option<ClickOffset> {
        if self.hovered(point) {
            Some(
                Point { 
                   x: point.x - self.location.x, 
                   y: point.y - self.location.y, 
                   z: self.location.z 
               }
            )
        } else {
            None
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}
