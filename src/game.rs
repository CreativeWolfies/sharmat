use std::num::NonZeroUsize;
use std::default::Default;
use crate::board::Board;
use crate::piece::Piece;
use crate::player::{Player, PlayerColor};

#[derive(Debug)]
pub struct Game {
    pieces: Vec<Piece>,
    board: Board,
    pub players: Vec<Player>,
    current_player: usize,
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

    pub fn set(&mut self, x: usize, y: usize, piece: &str, color: PlayerColor) -> Option<()> {
        let piece_index = self.pieces.iter().enumerate().find(|(_k, x)| x.id() == piece || x.alias_list().contains(&piece.to_string()))?.0;
        self.board.set(x, y, Some((piece_index, color))).ok()
    }

    pub fn player(&self, color: PlayerColor) -> Option<&Player> {
        self.players.iter().find(|p| p.color == color)
    }

    pub fn current_player(&self) -> Option<&Player> {
        self.players.get(self.current_player)
    }

    pub fn next_player(&mut self) {
        self.current_player += 1;
        if self.current_player >= self.players.len() {
            self.current_player = 0;
        }
    }
}

pub struct GameBuilder {
    game_pieces: Vec<Piece>,
    game_board: Board,
    game_players: Vec<Player>,
}

impl Default for GameBuilder {
    fn default() -> Self {
        GameBuilder {
            game_pieces: vec![],
            game_board: Board::new(NonZeroUsize::new(1).unwrap(), NonZeroUsize::new(1).unwrap()),
            game_players: vec![],
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

    pub fn player(mut self, player: Player) -> Self {
        self.game_players.push(player);
        self
    }

    pub fn build(self) -> Game {
        Game {
            board: self.game_board,
            pieces: self.game_pieces,
            players: self.game_players,
            current_player: 0,
        }
    }
}
