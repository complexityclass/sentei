#[derive(PartialEq, Debug, Clone)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn other(&self) -> Self {
        match self {
            Player::Black => Player::White,
            _ => Player::Black
        }
    }
}