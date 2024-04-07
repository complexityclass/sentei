extern crate core;

use crate::player::Player;

mod player;
mod point;
mod r#move;
mod stones_string;
mod board;

pub struct Game {
    pub human: Player,
    pub ai: Player,
}

impl Game {
    pub fn new(is_human_start: bool) -> Self {
        let human = if is_human_start { Player::Black } else { Player::White };
        let ai = human.other();
        Game {
            human,
            ai
        }
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
