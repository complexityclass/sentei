use crate::point::Point;

enum Move {
    Point(Point),
    Pass,
    Resign
}