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
    let board = Board::new(NonZeroUsize::new(8).unwrap(), NonZeroUsize::new(8).unwrap());
    let mut game = GameBuilder::new()
        .board(board)
        .piece(
            PieceBuilder::new()
            .id("standard.bishop")
            .alias("bishop")
            .display_white("standard.w_bishop")
            .display_black("standard.b_bishop")
            .movement(vec![MovementType::RangeAny(Box::new(MovementType::Undirected(1, 1)))])
            .build()
        )
        .piece(
            PieceBuilder::new()
            .id("standard.rook")
            .alias("rook")
            .display_white("standard.w_rook")
            .display_black("standard.b_rook")
            .movement(vec![MovementType::RangeAny(Box::new(MovementType::Undirected(1, 0)))])
            .build()
        )
        .piece(
            PieceBuilder::new()
            .id("standard.king")
            .alias("king")
            .display_white("standard.w_king")
            .display_black("standard.b_king")
            .movement(vec![MovementType::Union(vec![MovementType::Undirected(1, 0), MovementType::Undirected(1, 1)])])
            .build()
        )
        .piece(
            PieceBuilder::new()
            .id("standard.queen")
            .alias("queen")
            .display_white("standard.w_queen")
            .display_black("standard.b_queen")
            .movement(vec![MovementType::RangeAny(Box::new(MovementType::Union(vec![MovementType::Undirected(1, 0), MovementType::Undirected(1, 1)])))])
            .build()
        )
        .piece(
            PieceBuilder::new()
            .id("standard.knight")
            .alias("knight")
            .display_white("standard.w_knight")
            .display_black("standard.b_knight")
            .movement(vec![MovementType::Undirected(2, 1)])
            .build()
        )
        .piece(
            PieceBuilder::new()
            .id("standard.pawn")
            .alias("pawn")
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

    game.set(0, 0, "rook", PlayerColor::White).unwrap();
    game.set(1, 0, "knight", PlayerColor::White).unwrap();
    game.set(2, 0, "bishop", PlayerColor::White).unwrap();
    game.set(3, 0, "king", PlayerColor::White).unwrap();
    game.set(4, 0, "queen", PlayerColor::White).unwrap();
    game.set(5, 0, "bishop", PlayerColor::White).unwrap();
    game.set(6, 0, "knight", PlayerColor::White).unwrap();
    game.set(7, 0, "rook", PlayerColor::White).unwrap();

    for x in 0..8 {
        game.set(x, 6, "pawn", PlayerColor::Black).unwrap();
        game.set(x, 1, "pawn", PlayerColor::White).unwrap();
    }

    game.set(0, 7, "rook", PlayerColor::Black).unwrap();
    game.set(1, 7, "knight", PlayerColor::Black).unwrap();
    game.set(2, 7, "bishop", PlayerColor::Black).unwrap();
    game.set(3, 7, "king", PlayerColor::Black).unwrap();
    game.set(4, 7, "queen", PlayerColor::Black).unwrap();
    game.set(5, 7, "bishop", PlayerColor::Black).unwrap();
    game.set(6, 7, "knight", PlayerColor::Black).unwrap();
    game.set(7, 7, "rook", PlayerColor::Black).unwrap();
    let piece_assets = pieces::load_assets(format!("{}/assets/", env!("CARGO_MANIFEST_DIR")));
    gui::Sharmat::run(Settings::with_flags((piece_assets, game)))
}
