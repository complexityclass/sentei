extern crate core;

use std::collections::HashMap;
use crate::agent::Agent;
use crate::game_state::GameState;
use crate::player::Player;
use crate::agents::RandomBot;
use crate::player::Player::White;
use crate::r#move::Move;

mod board;
mod game_state;
mod r#move;
mod player;
mod point;
mod stones_string;
mod agent;
mod agents;

pub struct Game {
    pub human: Player,
    pub ai: Player,
}

impl Game {
    pub fn new(is_human_start: bool) -> Self {
        let human = if is_human_start {
            Player::Black
        } else {
            Player::White
        };
        let ai = human.other();
        Game { human, ai }
    }
}

pub struct DemoGame {
    game_state: GameState,
    bots: HashMap<Player, RandomBot>,
}

impl Default for DemoGame {
    fn default() -> Self {
        DemoGame::new(5)
    }
}

#[derive(Default)]
pub struct Position {
    pub col: i32,
    pub row: i32,
}

pub struct Play {
    pub is_white: bool,
    pub position: Option<Position>,
    pub is_end: bool
}

impl DemoGame {
    pub fn new(board_size: i32) -> Self {
        let mut bots = HashMap::new();
        bots.insert(Player::Black, RandomBot {});
        bots.insert(Player::White, RandomBot {});

        DemoGame {
            game_state: GameState::new(board_size),
            bots: bots
        }
    }
    pub fn next_play(&mut self) -> Play {
        let next_move = self.bots.get(&self.game_state.next_player).unwrap().select_move(&self.game_state);
        let play = match next_move {
            Move::Pass => {
                Play {
                    is_white: &self.game_state.next_player == &White,
                    position: None,
                    is_end: false
                }
            },
            Move::Resign => {
                Play {
                    is_white: &self.game_state.next_player == &White,
                    position: None,
                    is_end: true
                }
            }
            Move::Point(p) => {
                Play {
                    is_white: &self.game_state.next_player == &White,
                    position: Some(Position {
                        col: p.col,
                        row: p.row,
                    }),
                    is_end: false
                }
            }
        };
        self.game_state = self.game_state.apply_move(next_move);
        play
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_other() {
        let pb = Player::Black;
        let pw = Player::White;
        assert_eq!(pb.other(), pw);
        assert_eq!(pw.other(), pb);
    }
}
