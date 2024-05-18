use crate::board::Board;
use crate::player::Player;
use crate::point::Point;

struct Agent {}

impl Agent {
    pub fn is_point_an_eye(board: &Board, point: Point, player: Player) -> bool {
        if board.get(point).is_some() {
            return false;
        }

        for neighbor in point.neighbors() {
            if board.is_on_grid(neighbor) {
                if let Some(nei_player) = board.get(neighbor) {
                    if player != nei_player {
                        return false;
                    }
                }
            }
        }

        let mut friendly_corners = 0;
        let mut off_board_corners = 0;
        let corners: Vec<Point> = vec![
            Point::new(point.row - 1, point.col - 1),
            Point::new(point.row - 1, point.col + 1),
            Point::new(point.row + 1, point.col - 1),
            Point::new(point.row + 1, point.col + 1),
        ];

        for corner in corners {
            if board.is_on_grid(corner) {
                if let Some(corner_player) = board.get(corner) {
                    if corner_player == player {
                        friendly_corners += 1;
                    }
                }
            } else {
                off_board_corners += 1;
            }
        }

        return if off_board_corners > 0 {
            off_board_corners + friendly_corners == 4
        } else {
            friendly_corners >= 3
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::player::Player::{Black, White};
    use crate::point::Point;

    #[test]
    fn test_is_point_an_eye() {
        let mut board = Board::new(5, 5);

        let stones = vec![
            Point::new(5, 2),
            Point::new(4, 2),
            Point::new(4, 3),
            Point::new(4, 4),
            Point::new(4, 5),
            Point::new(5, 4),
        ];

        for stone in &stones {
            board.place_stone(White, *stone);
        }

        board.place_stone(Black, Point::new(5, 1));

        println!("{:?}", &board);

        assert_eq!(
            Agent::is_point_an_eye(&board, Point::new(5, 2), White),
            false
        );
        assert_eq!(
            Agent::is_point_an_eye(&board, Point::new(4, 1), Black),
            false
        );
        assert_eq!(
            Agent::is_point_an_eye(&board, Point::new(5, 1), White),
            false
        );
        assert_eq!(
            Agent::is_point_an_eye(&board, Point::new(5, 3), White),
            true
        );
        assert_eq!(
            Agent::is_point_an_eye(&board, Point::new(5, 3), Black),
            false
        );
        assert_eq!(
            Agent::is_point_an_eye(&board, Point::new(5, 5), White),
            true
        );
        assert_eq!(
            Agent::is_point_an_eye(&board, Point::new(5, 5), Black),
            false
        );
    }
}
