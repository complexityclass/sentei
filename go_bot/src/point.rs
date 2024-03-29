#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub(crate) struct Point {
    row: i32,
    col: i32
}

impl Point {

    pub(crate) fn new(row: i32, col: i32) -> Self {
        Point { row, col }
    }

    fn neighbors(&self) -> Vec<Point> {
        vec![
            Point::new(self.row - 1, self.col),
            Point::new(self.row + 1, self.col),
            Point::new(self.row, self.col - 1),
            Point::new(self.row, self.col + 1)
        ]
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let point = Point::new(0, 0);
        let neighbors = point.neighbors();
        assert_eq!(neighbors.len(), 4);
        assert!(neighbors.contains(&Point::new(-1, 0)));
        assert!(neighbors.contains(&Point::new(1, 0)));
        assert!(neighbors.contains(&Point::new(0, -1)));
        assert!(neighbors.contains(&Point::new(0, 1)));
    }
}