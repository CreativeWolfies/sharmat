use super::board::Board;
use super::game::Game;
use super::piece::Piece;
use super::player::Player;
use std::fmt;

#[derive(Clone, Debug)]
pub enum MovementType {
    Stay,
    Undirected(usize, usize), // (a, b)-jumper
    Directed(isize, isize), // <a, b> -mover
    RangeAny(Box<MovementType>), // x-∞-rider
    Range(Box<MovementType>, usize), // x-n-rider
    Union(Vec<MovementType>), // a OR b
    Composition(Vec<MovementType>), // a THEN b
    Tag(Box<MovementType>, Vec<MovementTag>)
    // Custom?
}

pub enum MovementTag {
    Capture,
    CaptureWithoutMoving,
    NoCapture,
    ActionBefore(&'static (dyn Fn(&mut Game) -> () + 'static)),
    ActionAfter(&'static (dyn Fn(&mut Game) -> () + 'static)),
    Condition(&'static (dyn Fn(&Board, &Player, isize, isize) -> bool + 'static))
}

impl Copy for MovementTag {}

impl Clone for MovementTag {
    fn clone(&self) -> Self {
        match self {
            MovementTag::Capture => MovementTag::Capture,
            MovementTag::CaptureWithoutMoving => MovementTag::CaptureWithoutMoving,
            MovementTag::NoCapture => MovementTag::NoCapture,
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
            MovementTag::ActionBefore(_) => write!(f, "ActionBefore(<fn>)"),
            MovementTag::ActionAfter(_) => write!(f, "ActionAfter(<fn>)"),
            MovementTag::Condition(_) => write!(f, "Condition(<fn>)"),
        }
    }
}
