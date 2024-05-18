use crate::board::Board;
use crate::player::Player;
use crate::player::Player::Black;
use crate::r#move::Move;
use crate::r#move::Move::Pass;
use crate::r#move::Move::Resign;
use std::cmp::PartialEq;

#[derive(Clone)]
pub struct GameState {
    pub board: Board,
    pub next_player: Player,
    previous_state: Option<Box<GameState>>,
    last_move: Option<Move>,
}

impl GameState {
    pub fn new(size: i32) -> Self {
        GameState {
            board: Board::new(size, size),
            next_player: Black,
            previous_state: None,
            last_move: None,
        }
    }

    pub fn apply_move(&self, game_move: Move) -> GameState {
        let next_board = match game_move {
            Move::Point(p) => {
                let mut next_board = self.board.clone();
                next_board.place_stone(self.next_player, p);
                next_board
            }
            Move::Resign => {
                println!("Resigned!");
                self.board.clone()
            }
            Move::Pass => self.board.clone(),
        };

        GameState {
            board: next_board,
            next_player: self.next_player.other(),
            previous_state: Some(Box::new(self.clone())),
            last_move: Some(game_move.clone()),
        }
    }

    pub fn is_valid_move(&self, mv: Move) -> bool {
        if self.is_over() {
            return false;
        }

        return match mv {
            Pass => true,
            Resign => true,
            Move::Point(point) => {
                self.board.get_string(point).is_none()
                    && !self.is_move_self_capture(self.next_player, mv)
                    && !self.does_move_violate_ko(self.next_player, mv)
            }
        };
    }

    pub fn is_over(&self) -> bool {
        match &self.last_move {
            None => false,
            Some(mv) => match mv {
                Move::Resign => true,
                _ => match &self.previous_state {
                    None => false,
                    Some(previous_state) => {
                        if let Some(second_last_move) = &previous_state.last_move.clone() {
                            return second_last_move == &Move::Pass && mv == &Move::Pass;
                        } else {
                            return false;
                        }
                    }
                },
            },
        }
    }

    fn is_move_self_capture(&self, player: Player, mv: Move) -> bool {
        return if let Move::Point(p) = mv {
            let mut next_board = self.board.clone();
            next_board.place_stone(player, p);
            let next_string = next_board.get_string(p);
            next_string.is_some() && next_string.unwrap().liberties_count() == 0
        } else {
            false
        };
    }

    fn situation(&self) -> (Player, Board) {
        return (self.next_player, self.board.clone());
    }

    fn does_move_violate_ko(&self, player: Player, mv: Move) -> bool {
        return match mv {
            Move::Point(point) => {
                let mut next_board = self.board.clone();
                next_board.place_stone(player, point);
                let next_situation = (player.other(), next_board);
                let mut past_state = self.previous_state.as_ref();

                while let Some(pstate) = past_state {
                    if pstate.situation() == next_situation {
                        return true;
                    }
                    past_state = pstate.previous_state.as_ref();
                }

                false
            }
            _ => false,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::player::Player::{Black, White};
    use crate::point::Point;
    use crate::r#move::Move::Point as PointMove;

    use super::*;

    #[test]
    fn test_create_game() {
        let game_state = GameState::new(9);
        assert_eq!(game_state.next_player, Black);
    }

    #[test]
    fn test_is_over_after_resign() {
        let game_state = GameState::new(3);
        assert!(!game_state.is_over());
        assert!(game_state.apply_move(Move::Resign).is_over());
    }

    #[test]
    fn test_not_self_capture() {
        let mut board = Board::new(2, 2);
        board.place_stone(Black, Point::new(1, 1));

        let game_state = GameState {
            board,
            next_player: Player::White,
            previous_state: None,
            last_move: None,
        };

        assert!(!game_state.is_move_self_capture(Black, PointMove(Point::new(1, 2))));
    }

    #[test]
    fn test_self_capture() {
        let mut board = Board::new(3, 3);
        board.place_stone(Black, Point::new(2, 1));
        board.place_stone(Black, Point::new(2, 2));
        board.place_stone(Black, Point::new(3, 2));

        println!("{:?}", board);

        let game_state = GameState {
            board,
            next_player: Player::White,
            previous_state: None,
            last_move: None,
        };

        assert!(game_state.is_move_self_capture(White, PointMove(Point::new(3, 1))));
    }

    #[test]
    fn test_apply_move() {
        let mut game_state = GameState::new(3);
        let mv: Move = PointMove(Point::new(1, 1));
        game_state = game_state.apply_move(mv);
        assert_eq!(game_state.next_player, White);
        let state_one_str: String = format!("{:?}", game_state.board);
        assert_eq!(
            state_one_str,
            String::from("┌──────────┐\n│ ●  .  . │\n│ .  .  . │\n│ .  .  . │\n└──────────┘\n")
        );
        game_state = game_state.apply_move(PointMove(Point::new(2, 1)));
        assert_eq!(
            format!("{:?}", game_state.board),
            String::from("┌──────────┐\n│ ●  .  . │\n│ ○  .  . │\n│ .  .  . │\n└──────────┘\n")
        );
    }

    #[test]
    fn test_move_violates_ko() {
        let board = Board::new(5, 5);

        let moves = vec![
            Point::new(2, 2),
            Point::new(2, 3),
            Point::new(3, 1),
            Point::new(3, 4),
            Point::new(4, 2),
            Point::new(4, 3),
            Point::new(3, 3),
            Point::new(3, 2),
        ];

        let mut game_state = GameState {
            board,
            next_player: Player::Black,
            previous_state: None,
            last_move: None,
        };

        for mv in moves {
            assert!(!game_state.does_move_violate_ko(game_state.next_player, Move::Point(mv)));
            game_state = game_state.apply_move(Move::Point(mv));
        }

        assert!(game_state.does_move_violate_ko(Black, Move::Point(Point::new(3, 3))));
    }

    #[test]
    fn test_is_valid_move() {
        let board = Board::new(5, 5);

        let moves = vec![
            Point::new(2, 2),
            Point::new(2, 3),
            Point::new(3, 1),
            Point::new(3, 4),
            Point::new(4, 2),
            Point::new(4, 3),
            Point::new(3, 3),
            Point::new(3, 2),
        ];

        let mut game_state = GameState {
            board,
            next_player: Player::Black,
            previous_state: None,
            last_move: None,
        };

        for mv in moves {
            assert!(game_state.is_valid_move(Move::Point(mv)));
            game_state = game_state.apply_move(Move::Point(mv));
        }

        assert!(!game_state.is_valid_move(Move::Point(Point::new(2, 2))));
        assert!(!game_state.is_valid_move(Move::Point(Point::new(3, 3))));
    }
}
