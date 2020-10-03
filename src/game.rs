use std::default::Default;
use crate::board::Board;
use crate::piece::Piece;

pub struct Game<'a> {
    game_pieces: Vec<&'a Piece>,
    game_boards: Vec<&'a Board<'a>>,
}

impl<'a> Game<'a> {
    pub fn pieces(&self) -> &Vec<&Piece> {
        &self.game_pieces
    }

    pub fn boards(&self) -> &Vec<&Board> {
        &self.game_boards
    }

    pub fn search_piece(&self, id: &str) -> Option<&&Piece> {
        self.game_pieces.iter().find(|x| x.id() == id)
    }

    pub fn search_piece_alias(&self, alias: &str) -> Vec<&&Piece> {
        self.game_pieces.iter().filter(|x| x.alias_list().contains(&alias.to_string())).collect()
    }
}

pub struct GameBuilder<'a> {
    game_pieces: Vec<&'a Piece>,
    game_boards: Vec<&'a Board<'a>>,
}

impl Default for GameBuilder<'_> {
    fn default() -> Self {
        GameBuilder {
            game_pieces: vec![],
            game_boards: vec![],
        }
    }
}

impl<'a> GameBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn board(mut self, board: &'a Board<'a>) -> Self {
        self.game_boards.push(board);
        self
    }

    pub fn boards(mut self, mut boards: Vec<&'a Board<'a>>) -> Self {
        self.game_boards.append(&mut boards);
        self
    }

    pub fn piece(mut self, piece: &'a Piece) -> Self {
        self.game_pieces.push(piece);
        self
    }

    pub fn pieces(mut self, mut pieces: Vec<&'a Piece>) -> Self {
        self.game_pieces.append(&mut pieces);
        self
    }

    pub fn build(self) -> Game<'a> {
        Game {
            game_boards: self.game_boards,
            game_pieces: self.game_pieces,
        }
    }
}
