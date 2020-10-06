use std::num::NonZeroUsize;
use crate::piece::Piece;
use self::BoardError::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Board<'a> {
    pub width: NonZeroUsize,
    pub height: NonZeroUsize,
    board: Vec<Vec<Option<&'a Piece>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoardError {
    OutOfBounds(usize, usize),
}

pub type BoardResult<T> = Result<T, BoardError>;

impl<'a> Board<'a> {
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
        }
    }

    pub fn set(&mut self, x: usize, y: usize, piece: &'a Piece) -> BoardResult<()> {
        let res_check_pos = self.check_pos(x, y);
        if res_check_pos.is_err() {
            return res_check_pos;
        }
        self.board[x][y] = Some(piece);
        Ok(())
    }

    pub fn get(&self, x: usize, y: usize) -> BoardResult<Option<&Piece>> {
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

    pub fn set_name(&mut self, name: &'a str) {
        unimplemented!();
    }

    pub fn name(&self) -> &'a str {
        unimplemented!();
    }
}
