#[allow(dead_code)]
#[derive(Clone)]
pub enum Component {
    BoxListFull,
    BoxListHollow,
    CircleListFull,
    CircleListHollow,
    Heading1,
    Heading2,
    Heading3,
    Text,
}
impl Default for Component {
    fn default() -> Self {
        Component::Text
    }
}
