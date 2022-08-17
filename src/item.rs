use crate::component::Component;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Item {
    component: Component,
    pub content: String,
    pub position: (u16, u16),
    pub lines: u16,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            component: Component::default(),
            content: String::from("Default content"),
            position: (1, 1),
            lines: 1,
        }
    }
}
impl Item {
    pub fn new_at_y(y: u16) -> Self {
        Item {
            component: Component::default(),
            content: String::from("Default content"),
            position: (1, y),
            lines: 1,
        }
    }
}
