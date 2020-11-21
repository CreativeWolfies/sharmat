use sharmat::board::*;
use sharmat::game::*;
use sharmat::movement::*;
use sharmat::piece::*;
use sharmat::player::*;
use sharmat::rule::*;
use std::num::NonZeroUsize;

fn assert_set_equal<T: PartialEq + std::fmt::Debug>(a: Vec<T>, b: Vec<T>) {
    let res = set_equal(&a, &b);
    assert!(
        res,
        "Sets A and B are not equal:\nA = {:#?}\nB = {:#?}",
        a, b
    )
}

/// Returns `a = b <=> a ⊂ b & b ⊂ a`
fn set_equal<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    set_inclusion(a, b) && set_inclusion(b, a)
}

/// Returns `a ⊂ b`
/// O(n²), I don't care
fn set_inclusion<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    a.len() == 0
        || a.iter()
            .all(|a_elem| b.iter().find(|b_elem| a_elem == *b_elem).is_some())
}

#[test]
fn movement_undirected() {
    let knight_movement = MovementType::Undirected(2, 1);
    let board = Board::new(NonZeroUsize::new(8).unwrap(), NonZeroUsize::new(8).unwrap());
    let player = Player::new(PlayerColor::White);
    assert_set_equal(
        knight_movement.flatten(&board, &player, 4, 4).unwrap(),
        vec![
            (2, 1),
            (1, 2),
            (-2, 1),
            (-1, 2),
            (2, -1),
            (1, -2),
            (-2, -1),
            (-1, -2),
        ],
    );
}

#[test]
fn movement_directed() {
    let pawn_movement = MovementType::Directed(0, 1);
    let board = Board::new(NonZeroUsize::new(8).unwrap(), NonZeroUsize::new(8).unwrap());
    let player = Player::new(PlayerColor::White);
    assert_set_equal(
        pawn_movement.flatten(&board, &player, 4, 4).unwrap(),
        vec![(0, 1)],
    );
}

#[test]
fn movement_range() {
    let double_wazir_movement = MovementType::Range(Box::new(MovementType::Undirected(1, 0)), 2);
    let board = Board::new(NonZeroUsize::new(8).unwrap(), NonZeroUsize::new(8).unwrap());
    let player = Player::new(PlayerColor::White);
    assert_set_equal(
        double_wazir_movement
            .flatten(&board, &player, 4, 4)
            .unwrap(),
        vec![
            (1, 0),
            (2, 0),
            (-1, 0),
            (-2, 0),
            (0, 1),
            (0, 2),
            (0, -1),
            (0, -2),
        ],
    );
}

#[test]
fn movement_range_any() {
    let rook_movement = MovementType::RangeAny(Box::new(MovementType::Undirected(1, 0)));
    let board = Board::new(NonZeroUsize::new(8).unwrap(), NonZeroUsize::new(8).unwrap());
    let player = Player::new(PlayerColor::White);
    assert_set_equal(
        rook_movement.flatten(&board, &player, 4, 3).unwrap(),
        vec![
            (1, 0),
            (2, 0),
            (3, 0),
            (-1, 0),
            (-2, 0),
            (-3, 0),
            (-4, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, -1),
            (0, -2),
            (0, -3),
        ],
    );
}
