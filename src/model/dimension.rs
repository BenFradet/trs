#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Dimension {
    Row,
    Col,
}

impl Dimension {
    pub fn inverse(self) -> Self {
        if self == Dimension::Col {
            Dimension::Row
        } else {
            Dimension::Col
        }
    }
}
