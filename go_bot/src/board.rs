use core::fmt;
use std::collections::HashMap;
use std::fmt::Formatter;

use crate::player::Player;
use crate::point::Point;
use crate::stones_string::StonesString;

pub struct Board {
    pub num_rows: i32,
    pub num_cols: i32,
    grid: HashMap<Point, StonesString>,
}

impl Board {
    pub fn new(rows: i32, cols: i32) -> Self {
        Board {
            num_rows: rows,
            num_cols: cols,
            grid: HashMap::new(),
        }
    }

    pub fn place_stone(&mut self, player: Player, point: Point) {
        if !self.is_on_grid(point) {
            panic!("Item out of the board size")
        }

        if !self.get(point).is_none() {
            panic!("Stone can't be placed at the already taken position")
        }

        let mut adj_same_color: Vec<StonesString> = vec![];
        let mut adj_opposite_color: Vec<StonesString> = vec![];
        let mut liberties = vec![];

        for neighbor in point.neighbors() {
            if !self.is_on_grid(neighbor) {
                continue;
            }

            if let Some(neighbor_string) = self.grid.get(&neighbor) {
                if neighbor_string.color == player {
                    if !adj_same_color.contains(&neighbor_string) {
                        adj_same_color.push(neighbor_string.clone());
                    }
                } else {
                    if !adj_opposite_color.contains(&neighbor_string) {
                        adj_opposite_color.push(neighbor_string.clone());
                    }
                }
            } else {
                liberties.push(neighbor);
            }
        }

        let mut new_string = StonesString::new(player, &*vec![point], &liberties);

        for same_color_string in adj_same_color {
            new_string = new_string.merge(&same_color_string);
        }

        for new_string_point in &new_string.stones {
            self.grid
                .insert(new_string_point.clone(), new_string.clone());
        }

        for mut other_color_string in &mut adj_opposite_color {
            other_color_string.remove_liberty(&point);
            for stone in &other_color_string.stones {
                self.grid.insert(stone.clone(), other_color_string.clone());
            }
        }

        for other_color_string in &adj_opposite_color {
            if other_color_string.liberties_count() == 0 {
                self.remove_string(other_color_string);
            }
        }
    }

    fn remove_string(&mut self, string: &StonesString) {
        for point in &string.stones {
            for neighbor in point.neighbors() {
                if let Some(neighbor_string) = self.grid.get_mut(&neighbor) {
                    if neighbor_string != string {
                        neighbor_string.add_liberty(&point);
                    }
                } else {
                    continue;
                }
            }
            self.grid.remove(&point);
        }
    }

    fn is_on_grid(&self, point: Point) -> bool {
        return 1 <= point.row
            && point.row <= self.num_rows
            && 1 <= point.col
            && point.col <= self.num_cols;
    }

    fn get(&self, point: Point) -> Option<Player> {
        let string = self.grid.get(&point)?;
        Some(string.color.clone())
    }

    fn get_string(&self, point: Point) -> Option<&StonesString> {
        let string = self.grid.get(&point)?;
        Some(string)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let horizontal_line = "─".repeat((self.num_cols * 3 + 1) as usize);
        writeln!(f, "┌{}┐", horizontal_line)?;
        for i in 1..self.num_rows + 1 {
            write!(f, "│")?;
            for j in 1..self.num_cols + 1 {
                let point = Point { row: i, col: j };
                let cell = match self.get_string(point) {
                    Some(string) => match string.color {
                        Player::Black => "●",
                        Player::White => "○",
                        _ => " ",
                    },
                    None => ".",
                };
                write!(f, " {} ", cell)?;
            }
            writeln!(f, "│")?;
        }
        writeln!(f, "└{}┘", horizontal_line)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::player::Player::{Black, White};

    use super::*;

    #[test]
    #[should_panic(expected = "Item out of the board size")]
    fn test_throws_exception_if_out_of_grid() {
        let mut board = Board::new(9, 9);
        board.place_stone(White, Point::new(10, 10));
    }

    #[test]
    #[should_panic(expected = "Item out of the board size")]
    fn test_throws_exception_if_out_of_grid2() {
        let mut board = Board::new(9, 9);
        board.place_stone(White, Point::new(0, 1));
    }

    #[test]
    fn test_place_stone() {
        let mut board = Board::new(4, 4);
        board.place_stone(White, Point::new(1, 1));
        for i in 0..4 {
            for j in 0..4 {
                let p = Point::new(i, j);
                if i == 1 && j == 1 {
                    assert_eq!(board.get(p), Some(White));
                } else {
                    assert!(board.get(p).is_none());
                }
            }
        }
    }

    #[test]
    fn test_place_stones_in_string() {
        let mut board = Board::new(4, 4);
        let black_stones = vec![
            Point::new(1, 1),
            Point::new(1, 2),
            Point::new(1, 3),
            Point::new(2, 1),
        ];
        let white_stones = vec![Point::new(4, 2), Point::new(4, 3), Point::new(4, 4)];

        // Place Black stones
        for stone in &black_stones {
            board.place_stone(Black, *stone);
        }

        // Place White stones
        for stone in &white_stones {
            board.place_stone(White, *stone);
        }

        // Check Black string
        let black_string = board.get_string(Point::new(1, 1)).unwrap();
        for stone in &black_stones {
            assert_eq!(
                black_string,
                board.get_string(*stone).unwrap(),
                "Black stone at {:?} should be part of the same string",
                stone
            );
        }

        // Check White string
        let white_string = board.get_string(Point::new(4, 2)).unwrap();
        for stone in &white_stones {
            assert_eq!(
                white_string,
                board.get_string(*stone).unwrap(),
                "White stone at {:?} should be part of the same string",
                stone
            );
        }

        // Check for no stones at other points
        for i in 1..=4 {
            for j in 1..=4 {
                let p = Point::new(i, j);
                if !black_stones.contains(&p) && !white_stones.contains(&p) {
                    assert!(
                        board.get_string(p).is_none(),
                        "Point {:?} should not belong to any string",
                        p
                    );
                }
            }
        }
    }

    #[test]
    fn test_liberty_reduce() {
        let mut board = Board::new(2, 2);
        board.place_stone(White, Point::new(1, 1));
        assert_eq!(board.get(Point::new(1, 1)), Some(White));

        board.place_stone(Black, Point::new(1, 2));
        assert_eq!(board.get(Point::new(1, 2)), Some(Black));

        board.place_stone(Black, Point::new(2, 1));
        assert_eq!(board.get(Point::new(2, 1)), Some(Black));
    }

    #[test]
    fn test_capture() {
        let mut board = Board::new(4, 4);
        // Put white stone in the middle surrounded by 3 black stones
        let wp = Point::new(2, 2);
        board.place_stone(White, wp);

        let black_stones = vec![Point::new(1, 2), Point::new(2, 1), Point::new(2, 3)];

        for stone in &black_stones {
            board.place_stone(Black, *stone);
        }

        assert_eq!(board.get(wp), Some(White));
        assert_eq!(board.get(Point::new(2, 1)), Some(Black));
        assert_eq!(
            board.get_string(Point::new(2, 1)).unwrap().liberties.len(),
            2
        );

        // Add 4th black stone
        board.place_stone(Black, Point::new(3, 2));

        // White stone captured
        assert!(board.get(wp).is_none());

        // Black has one more liberty
        assert_eq!(
            board.get_string(Point::new(2, 1)).unwrap().liberties.len(),
            3
        );
    }
}
