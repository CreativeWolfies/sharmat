use std::default::Default;
use crate::board::Board;
use crate::piece::Piece;

#[derive(Debug)]
pub struct Game {
    game_pieces: Vec<Piece>,
    game_boards: Vec<Board>,
}

impl Game {
    pub fn pieces(&self) -> &Vec<Piece> {
        &self.game_pieces
    }

    pub fn boards(&self) -> &Vec<Board> {
        &self.game_boards
    }

    pub fn search_piece<'a>(&'a self, id: &str) -> Option<&'a Piece> {
        self.game_pieces.iter().find(|x| x.id() == id)
    }

    pub fn search_piece_alias<'a>(&'a self, alias: &str) -> Vec<&'a Piece> {
        self.game_pieces.iter().filter(|x| x.alias_list().contains(&alias.to_string())).collect()
    }
}

pub struct GameBuilder {
    game_pieces: Vec<Piece>,
    game_boards: Vec<Board>,
}

impl Default for GameBuilder {
    fn default() -> Self {
        GameBuilder {
            game_pieces: vec![],
            game_boards: vec![],
        }
    }
}

impl GameBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn board(mut self, board: Board) -> Self {
        self.game_boards.push(board);
        self
    }

    pub fn boards(mut self, mut boards: Vec<Board>) -> Self {
        self.game_boards.append(&mut boards);
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
            game_boards: self.game_boards,
            game_pieces: self.game_pieces,
        }
    }
}
