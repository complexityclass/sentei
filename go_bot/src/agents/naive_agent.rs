use crate::agent::{is_point_an_eye, Agent};
use crate::game_state::GameState;
use crate::point::Point;
use crate::r#move::Move;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct RandomBot;

impl Agent for RandomBot {
    fn select_move(&self, game_state: &GameState) -> Move {
        let mut candidates = vec![];
        for r in 1..game_state.board.num_rows + 1 {
            for c in 1..game_state.board.num_cols + 1 {
                let candidate = Point::new(r, c);
                if game_state.is_valid_move(Move::Point(candidate))
                    && !is_point_an_eye(&game_state.board, candidate, game_state.next_player)
                {
                    candidates.push(candidate);
                }
            }
        }

        return if candidates.is_empty() {
            Move::Pass
        } else {
            let mut rng = thread_rng();
            Move::Point(*candidates.choose(&mut rng).unwrap())
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::Agent;
    use crate::game_state::GameState;
    use crate::player::Player;
    use std::collections::HashMap;

    #[test]
    fn test_random_bot_new() {
        let board_size = 6;
        let mut game = GameState::new(board_size);
        let mut bots = HashMap::new();
        bots.insert(Player::Black, RandomBot {});
        bots.insert(Player::White, RandomBot {});

        let mut steps = 0;

        while !game.is_over() && steps < 300 {
            steps += 1;
            let next_move = bots.get(&game.next_player).unwrap().select_move(&game);
            match next_move {
                Move::Pass => println!("{:?} passing", &game.next_player),
                Move::Resign => println!("{:?} resigns", &game.next_player),
                _ => println!("{:?}", game.board)
            }

            game = game.apply_move(next_move);
        }

        println!("Game finished in {:?} moves", steps);
    }

    #[test]
    fn test_random_bot_select_move() {}
}
