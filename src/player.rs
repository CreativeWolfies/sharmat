#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PlayerColor {
    White,
    Black,
}

#[derive(Debug)]
pub struct Player {
    pub color: PlayerColor,
}

impl PlayerColor {
    pub fn white(&self) -> bool {
        *self == PlayerColor::White
    }

    pub fn black(&self) -> bool {
        *self == PlayerColor::Black
    }
}

impl Player {
    pub fn new(color: PlayerColor) -> Self {
        Self { color }
    }
}
