use crate::player::Player;
use crate::point::Point;
use crate::stones_string::StonesString;
use std::collections::HashMap;

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
        }

        for other_color_string in &adj_opposite_color {
            if other_color_string.liberties_count() == 0 {
                todo!("remove string")
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Item out of the board size")]
    fn test_throws_exception_if_out_of_grid() {
        let mut board = Board::new(9, 9);
        board.place_stone(Player::White, Point::new(10, 10));
    }
}
