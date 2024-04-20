use crate::board::Board;
use crate::player::Player;
use crate::player::Player::Black;
use crate::r#move::Move;
use crate::r#move::Move::Pass;
use std::cmp::PartialEq;

#[derive(Clone)]
struct GameState {
    board: Board,
    next_player: Player,
    previous_state: Option<Box<GameState>>,
    last_move: Option<Move>,
}

impl GameState {
    fn new(size: i32) -> Self {
        GameState {
            board: Board::new(size, size),
            next_player: Black,
            previous_state: None,
            last_move: None,
        }
    }

    fn apply_move(&self, game_move: Move) -> GameState {
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

    fn is_over(&self) -> bool {
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
        if let Move::Point(p) = mv {
            let mut next_board = self.board.clone();
            println!("{:?}", next_board);
            next_board.place_stone(player, p);
            let next_string = next_board.get_string(p);
            return next_string.is_some() && next_string.unwrap().liberties_count() == 0;
        } else {
            return false;
        }
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
}
