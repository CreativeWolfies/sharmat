use std::num::NonZeroUsize;
use std::default::Default;
use crate::board::Board;
use crate::piece::Piece;

#[derive(Debug)]
pub struct Game {
    pieces: Vec<Piece>,
    board: Board,
}

impl Game {
    pub fn pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn search_piece<'a>(&'a self, id: &str) -> Option<&'a Piece> {
        self.pieces.iter().find(|x| x.id() == id)
    }

    pub fn search_piece_alias<'a>(&'a self, alias: &str) -> Vec<&'a Piece> {
        self.pieces.iter().filter(|x| x.alias_list().contains(&alias.to_string())).collect()
    }
}

pub struct GameBuilder {
    game_pieces: Vec<Piece>,
    game_board: Board,
}

impl Default for GameBuilder {
    fn default() -> Self {
        GameBuilder {
            game_pieces: vec![],
            game_board: Board::new(NonZeroUsize::new(1).unwrap(), NonZeroUsize::new(1).unwrap()),
        }
    }
}

impl GameBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn board(mut self, board: Board) -> Self {
        self.game_board = board;
        self
    }

    pub fn piece(mut self, piece: Piece) -> Self {
        self.game_pieces.push(piece);
        self
    }

    pub fn pieces(mut self, mut pieces: Vec<Piece>) -> Self {
        self.game_pieces.append(&mut pieces);
        self
    }

    pub fn build(self) -> Game {
        Game {
            board: self.game_board,
            pieces: self.game_pieces,
        }
    }
}
