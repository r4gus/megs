use crate::misc::Point;
use uuid::Uuid;

pub type ClickOffset = Point;

pub trait Hoverable {
    fn hovered(&self, point: Point) -> bool;
}

pub trait Clickable: Hoverable {
    fn clicked(&mut self, point: Point) -> Option<ClickOffset>;
    fn selectable(&self) -> bool;
}

pub trait Draggable: Clickable {
    fn dragged(&mut self, point: Point);
    fn released();
}
