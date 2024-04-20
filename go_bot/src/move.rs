use crate::point::Point;

#[derive(Clone, PartialEq)]
pub enum Move {
    Point(Point),
    Pass,
    Resign
}