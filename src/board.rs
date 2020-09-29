use std::num::NonZeroUsize;
use crate::piece::Piece;
use self::BoardError::*;

#[derive(Debug)]
pub struct Board {
    width: NonZeroUsize,
    height: NonZeroUsize,
    board: Vec<Vec<Option<Piece>>>,
}

#[derive(Debug)]
pub enum BoardError {
    PieceOutOfBounds,
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
        }
    }

    pub fn set(&mut self, x: usize, y: usize, piece: Piece) -> BoardResult<()> {
        let res_check_pos = self.check_pos(x, y);
        if res_check_pos.is_err() {
            return res_check_pos;
        }
        self.board[x][y] = Some(piece);
        Ok(())
    }

    pub fn get(&self, x: usize, y: usize) -> BoardResult<&Option<Piece>> {
        let res_check_pos = self.check_pos(x, y);
        if let Err(err) = res_check_pos {
            return Err(err);
        }
        Ok(&self.board[x][y])
    }

    fn check_pos(&self, x: usize, y: usize) -> BoardResult<()> {
        if x >= self.width.get() - 1 || y >= self.height.get() - 1 {
            return Err(PieceOutOfBounds);
        }
        Ok(())
    }
}
