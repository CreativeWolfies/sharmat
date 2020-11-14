use std::num::NonZeroUsize;
use self::BoardError::*;
use super::player::PlayerColor;

type RawPiece = Option<(usize, PlayerColor)>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Board {
    pub width: NonZeroUsize,
    pub height: NonZeroUsize,
    board: Vec<Vec<RawPiece>>,
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoardError {
    OutOfBounds(usize, usize),
}

pub type BoardResult<T> = Result<T, BoardError>;

impl Board {
    pub fn new(width: NonZeroUsize, height: NonZeroUsize) -> Self {
        let mut board = Vec::with_capacity(width.get());
        for x in 0..width.get() {
            board.push(Vec::with_capacity(height.get()));
            for _y in 0..height.get() {
                board[x].push(None);
            }
        }
        Board {
            width,
            height,
            board,
            name: String::from("Board")
        }
    }

    pub fn set(&mut self, x: usize, y: usize, piece: RawPiece) -> BoardResult<()> {
        self.check_pos(x, y)?;
        self.board[x][y] = piece;
        Ok(())
    }

    pub fn get(&self, x: usize, y: usize) -> BoardResult<RawPiece> {
        self.check_pos(x, y)?;
        Ok(self.board[x][y])
    }

    pub fn move_piece(&mut self, x: usize, y: usize, dx: usize, dy: usize) -> BoardResult<()> {
        self.check_pos(x, y)?;
        self.check_pos(dx, dy)?;
        self.board[dx][dy] = self.board[x][y];
        self.board[x][y] = None;
        Ok(())
    }

    pub fn clear_pos(&mut self, x: usize, y: usize) -> BoardResult<()> {
        self.check_pos(x, y)?;
        self.board[x][y] = None;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.board.iter_mut().for_each(|column| {
            column.iter_mut().for_each(|cell| {
                *cell = None;
            });
        });
    }

    fn check_pos(&self, x: usize, y: usize) -> BoardResult<()> {
        if x >= self.width.get() || y >= self.height.get() {
            println!("{}:{} / {}:{}", x, self.width.get(), y, self.height.get());
            return Err(OutOfBounds(x, y));
        }
        Ok(())
    }

    pub fn set_name<'a>(&'a mut self, name: &'a str) {
        self.name = name.to_string();
    }

    pub fn name<'a>(&'a self) -> String {
        self.name.clone()
    }
}
