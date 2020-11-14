// The ~client~ (the thing that displays pieces & stuff)

extern crate iced;
extern crate iced_native;
extern crate iced_wgpu;

use std::num::NonZeroUsize;
use iced::{Application, Settings};
use sharmat::board::Board;
use sharmat::piece::*;
use sharmat::game::*;
use sharmat::movement::*;
use sharmat::player::*;

pub mod gui;
pub mod style;
pub mod pieces;

fn main() {
    // TODO: do this in another thread or idk
    let mut board = Board::new(NonZeroUsize::new(8).unwrap(), NonZeroUsize::new(8).unwrap());
    board.set(0, 0, Some((1, PlayerColor::White))).unwrap();
    board.set(1, 0, Some((4, PlayerColor::White))).unwrap();
    board.set(2, 0, Some((0, PlayerColor::White))).unwrap();
    board.set(3, 0, Some((2, PlayerColor::White))).unwrap();
    board.set(4, 0, Some((3, PlayerColor::White))).unwrap();
    board.set(5, 0, Some((0, PlayerColor::White))).unwrap();
    board.set(6, 0, Some((4, PlayerColor::White))).unwrap();
    board.set(7, 0, Some((1, PlayerColor::White))).unwrap();

    for x in 0..8 {
        board.set(x, 6, Some((5, PlayerColor::Black))).unwrap();
        board.set(x, 1, Some((5, PlayerColor::White))).unwrap();
    }

    board.set(0, 7, Some((1, PlayerColor::Black))).unwrap();
    board.set(1, 7, Some((4, PlayerColor::Black))).unwrap();
    board.set(2, 7, Some((0, PlayerColor::Black))).unwrap();
    board.set(3, 7, Some((2, PlayerColor::Black))).unwrap();
    board.set(4, 7, Some((3, PlayerColor::Black))).unwrap();
    board.set(5, 7, Some((0, PlayerColor::Black))).unwrap();
    board.set(6, 7, Some((4, PlayerColor::Black))).unwrap();
    board.set(7, 7, Some((1, PlayerColor::Black))).unwrap();
    let game = GameBuilder::new()
        .board(board)
        .piece(
            PieceBuilder::new()
            .id("standard.bishop")
            .display_white("standard.w_bishop")
            .display_black("standard.b_bishop")
            .movement(vec![MovementType::RangeAny(Box::new(MovementType::Undirected(1, 1)))])
            .build()
        )
        .piece(
            PieceBuilder::new()
            .id("standard.rook")
            .display_white("standard.w_rook")
            .display_black("standard.b_rook")
            .movement(vec![MovementType::RangeAny(Box::new(MovementType::Undirected(1, 0)))])
            .build()
        )
        .piece(
            PieceBuilder::new()
            .id("standard.king")
            .display_white("standard.w_king")
            .display_black("standard.b_king")
            .movement(vec![MovementType::Union(vec![MovementType::Undirected(1, 0), MovementType::Undirected(1, 1)])])
            .build()
        )
        .piece(
            PieceBuilder::new()
            .id("standard.queen")
            .display_white("standard.w_queen")
            .display_black("standard.b_queen")
            .movement(vec![MovementType::RangeAny(Box::new(MovementType::Union(vec![MovementType::Undirected(1, 0), MovementType::Undirected(1, 1)])))])
            .build()
        )
        .piece(
            PieceBuilder::new()
            .id("standard.knight")
            .display_white("standard.w_knight")
            .display_black("standard.b_knight")
            .movement(vec![MovementType::Undirected(2, 1)])
            .build()
        )
        .piece(
            PieceBuilder::new()
            .id("standard.pawn")
            .display_white("standard.w_pawn")
            .display_black("standard.b_pawn")
            .movement(vec![MovementType::Union(vec![
                MovementType::Condition(Box::new(MovementType::Directed(0, 1)), vec![MovementCondition::AsWhite, MovementCondition::NoCapture]),
                MovementType::Condition(Box::new(MovementType::Directed(0, -1)), vec![MovementCondition::AsBlack, MovementCondition::NoCapture]),
                MovementType::Condition(
                    Box::new(MovementType::Union(vec![
                        MovementType::Directed(1, 1), MovementType::Directed(-1, 1)
                    ])),
                    vec![MovementCondition::AsWhite, MovementCondition::Capture]
                ),
                MovementType::Condition(
                    Box::new(MovementType::Union(vec![
                        MovementType::Directed(1, -1), MovementType::Directed(-1, -1)
                    ])),
                    vec![MovementCondition::AsBlack, MovementCondition::Capture]
                ),
                MovementType::Condition(Box::new(MovementType::Directed(0, 2)), vec![
                    MovementCondition::AsWhite,
                    MovementCondition::NoCapture,
                    MovementCondition::Custom(&|_b, _p, _x, y, _dx, _dy| {
                        y == 1
                    })
                ]),
                MovementType::Condition(Box::new(MovementType::Directed(0, -2)), vec![
                    MovementCondition::AsBlack,
                    MovementCondition::NoCapture,
                    MovementCondition::Custom(&|b, _p, _x, y, _dx, _dy| {
                        y == b.height.get() - 2
                    })
                ]),
                // TODO: en-passant :(
            ])])
            .build()
        )
        .build();
    let piece_assets = pieces::load_assets(format!("{}/assets/", env!("CARGO_MANIFEST_DIR")));
    gui::Sharmat::run(Settings::with_flags((piece_assets, game)))
}
