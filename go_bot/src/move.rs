use crate::point::Point;

#[derive(Clone, PartialEq)]
pub enum Move {
    Point(Point),
    Pass,
    Resign,
}

impl Move {
    pub fn is_play(&self) -> bool {
        return match self {
            Point => true,
            _ => false,
        };
    }
}
