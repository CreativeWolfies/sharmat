use super::board::Board;
use super::game::Game;
use super::player::Player;
use std::fmt;

#[derive(Clone, Debug)]
pub enum MovementType {
    Stay,
    Undirected(usize, usize), // (a, b)-jumper
    Directed(isize, isize), // <a, b> -mover
    RangeAny(Box<MovementType>), // x-∞-rider; may not contain any Composition: make it not recursive?
    Range(Box<MovementType>, usize), // x-n-rider; may not contain any Composition: make it not recursive?
    Union(Vec<MovementType>), // a OR b OR c ... OR ω
    Composition(Vec<MovementType>), // a THEN b THEN c ... THEN ω
    Tag(Box<MovementType>, Vec<MovementTag>)
    // Custom?
}

pub enum MovementTag {
    Capture,
    CaptureWithoutMoving,
    NoCapture,
    AsWhite,
    AsBlack,
    ActionBefore(&'static (dyn Fn(&mut Game) -> () + 'static)),
    ActionAfter(&'static (dyn Fn(&mut Game) -> () + 'static)),
    Condition(&'static (dyn Fn(&Board, &Player, isize, isize) -> bool + 'static))
}

type RawMovement = (Vec<(isize, isize)>, Vec<MovementTag>);

impl Copy for MovementTag {}

impl Clone for MovementTag {
    fn clone(&self) -> Self {
        match self {
            MovementTag::Capture => MovementTag::Capture,
            MovementTag::CaptureWithoutMoving => MovementTag::CaptureWithoutMoving,
            MovementTag::NoCapture => MovementTag::NoCapture,
            MovementTag::AsWhite => MovementTag::AsWhite,
            MovementTag::AsBlack => MovementTag::AsBlack,
            MovementTag::ActionBefore(f) => MovementTag::ActionBefore(*f),
            MovementTag::ActionAfter(f) => MovementTag::ActionAfter(*f),
            MovementTag::Condition(f) => MovementTag::Condition(*f),
        }
    }
}

impl fmt::Debug for MovementTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MovementTag::Capture => write!(f, "Capture"),
            MovementTag::CaptureWithoutMoving => write!(f, "CaptureWithoutMoving"),
            MovementTag::NoCapture => write!(f, "NoCapture"),
            MovementTag::AsWhite => write!(f, "AsWhite"),
            MovementTag::AsBlack => write!(f, "AsBlack"),
            MovementTag::ActionBefore(_) => write!(f, "ActionBefore(<fn>)"),
            MovementTag::ActionAfter(_) => write!(f, "ActionAfter(<fn>)"),
            MovementTag::Condition(_) => write!(f, "Condition(<fn>)"),
        }
    }
}

impl MovementType {
    pub fn raw_movements(&self, x: usize, y: usize, board: &Board) -> Option<Vec<RawMovement>> {
        match self {
            MovementType::Stay => Some(vec![]),
            MovementType::Undirected(dx, dy) => {
                let mut res = vec![];
                let mut try_append = |dx: isize, dy: isize| {
                    if is_within_bounds(board, dx, dy) {
                        res.push((vec![(dx, dy)], vec![]));
                    }
                };
                if *dx == *dy {
                    try_append(*dx as isize, *dy as isize);
                    try_append(-(*dx as isize), *dy as isize);
                    try_append(*dx as isize, -(*dy as isize));
                    try_append(-(*dx as isize), -(*dy as isize));
                } else {
                    // hard-coded permutations; idc we're in 2d
                    try_append(*dx as isize, *dy as isize);
                    try_append(-(*dx as isize), *dy as isize);
                    try_append(*dx as isize, -(*dy as isize));
                    try_append(-(*dx as isize), -(*dy as isize));
                    try_append(*dy as isize, *dx as isize);
                    try_append(-(*dy as isize), *dx as isize);
                    try_append(*dy as isize, -(*dx as isize));
                    try_append(-(*dy as isize), -(*dx as isize));
                }
                Some(res)
            }
            MovementType::Directed(dx, dy) => {
                if is_within_bounds(board, x as isize + *dx, y as isize + *dy) {
                    Some(vec![(vec![(*dx, *dy)], vec![])])
                } else {
                    Some(vec![])
                }
            }
            MovementType::RangeAny(mv) => {
                let mut res = vec![];
                for child_movement in mv.raw_movements(x, y, board)?.into_iter() {
                    if child_movement.0.len() != 1 {
                        return None
                    }
                    let (dx, dy) = child_movement.0[0].clone();
                    let tags = child_movement.1.clone();
                    for mult in 1..=(board.width.get().max(board.height.get()) as isize) {
                        if is_within_bounds(board, x as isize + dx * mult, y as isize + dy * mult) {
                            res.push((vec![(dx * mult, dy * mult)], tags.clone()));
                        } else {
                            break;
                        }
                    }
                }
                Some(res)
            }
            MovementType::Range(mv, max_range) => {
                let mut res = vec![];
                for child_movement in mv.raw_movements(x, y, board)?.into_iter() {
                    if child_movement.0.len() != 1 {
                        return None
                    }
                    let (dx, dy) = child_movement.0[0].clone();
                    let tags = child_movement.1.clone();
                    for mult in 1..=(*max_range as isize) {
                        if is_within_bounds(board, x as isize + dx * mult, y as isize + dy * mult) {
                            res.push((vec![(dx * mult, dy * mult)], tags.clone()));
                        } else {
                            break;
                        }
                    }
                }
                Some(res)
            }
            MovementType::Union(moves) => {
                let mut res = vec![];
                for mv in moves {
                    for raw_mv in mv.raw_movements(x, y, board)?.into_iter() {
                        res.push(raw_mv);
                    }
                }
                Some(res)
            }
            MovementType::Composition(_moves) => {
                // idk how to do this, it'll have to be recursive so might as well remove the Vec from the type
                unimplemented!();
            }
            MovementType::Tag(mv, tags) => {
                let mut res = vec![];
                for raw_mv in mv.raw_movements(x, y, board)?.into_iter() {
                    res.push((raw_mv.0, (raw_mv.1).into_iter().chain(tags.iter().cloned()).collect()));
                }
                Some(res)
            }
        }
    }
}

#[inline]
fn is_within_bounds(board: &Board, x: isize, y: isize) -> bool {
    x >= 0 && x < board.width.get() as isize && y >= 0 && y < board.height.get() as isize
}
