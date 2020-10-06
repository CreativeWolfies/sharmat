use super::board::Board;
use super::game::Game;
use super::piece::Piece;
use super::player::Player;
use std::fmt;

pub enum MovementType {
    Stay,
    Step(Direction),
    Jump(isize, isize), // dx, dy
    Range(Vec<DirectionAndRange>),
    LimitedRange(Vec<DirectionAndRange>, usize),
    Repeat(Box<MovementType>, usize),
    CustomRange(Box<MovementType>),
    Composition(Vec<MovementType>),
    ConditionalMove(Box<MovementType>, Box<dyn Fn(&Piece, &Player) -> bool>),
    OnlyCapture(Box<MovementType>),
    CaptureAndMove(Box<MovementType>),
    CaptureWithoutMoving(Box<MovementType>),
    ActionBefore(Box<MovementType>, Box<dyn Fn(&mut Game) -> ()>),
    ActionAfter(Box<MovementType>, Box<dyn Fn(&mut Game) -> ()>),
    CustomJump(isize, isize, Box<dyn Fn(&Board, isize, isize) -> bool>),
    Custom(Box<dyn Fn(&Board, usize, usize) -> (isize, isize)>),
}

impl fmt::Debug for MovementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MovementType::Stay => write!(f, "Stay"),
            MovementType::Step(dir) => write!(f, "Step({:?})", dir),
            MovementType::Jump(dx, dy) => write!(f, "Jump({}, {})", dx, dy),
            MovementType::Range(r) => write!(f, "Range({:?})", r),
            MovementType::LimitedRange(r, n) => write!(f, "LimitedRange({:?}, {})", r, n),
            MovementType::Repeat(mv, n) => write!(f, "Repeat({:?}, {})", mv, n),
            MovementType::CustomRange(mv) => write!(f, "CustomRange({:?})", mv),
            MovementType::Composition(mvs) => write!(f, "Composition({:?})", mvs),
            MovementType::ConditionalMove(mv, _f) => write!(f, "ConditionalMove({:?}, <fn>)", mv),
            MovementType::OnlyCapture(mv) => write!(f, "OnlyCapture({:?})", mv),
            MovementType::CaptureAndMove(mv) => write!(f, "CaptureAndMove({:?})", mv),
            MovementType::CaptureWithoutMoving(mv) => write!(f, "CaptureWithoutMoving({:?})", mv),
            MovementType::ActionBefore(mv, _f) => write!(f, "ActionBefore({:?}, <fn>)", mv),
            MovementType::ActionAfter(mv, _f) => write!(f, "ActionAfter({:?}, <fn>)", mv),
            MovementType::CustomJump(dx, dy, _f) => write!(f, "CustomJump({}, {}, <fn>)", dx, dy),
            MovementType::Custom(_f) => write!(f, "Custom(<fn>)"),
        }
    }
}

impl PartialEq for MovementType {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    UpLeft,
    UpRight,
    Right,
    Left,
    Down,
    DownLeft,
    DownRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Range {
    Diagonal,
    Orthogonal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionAndRange {
    Direction(Direction),
    Range(Range),
}

impl MovementType {
    pub fn steps(&self) -> Vec<Direction> {
        unimplemented!();
    }

    pub fn jumps(&self) -> Vec<(isize, isize)> {
        unimplemented!();
    }

    pub fn ranges(&self) -> Vec<DirectionAndRange> {
        unimplemented!();
    }

    pub fn limited_ranges(&self) -> Vec<(DirectionAndRange, usize)> {
        unimplemented!();
    }

    pub fn repeats(&self) -> Vec<MovementType> {
        unimplemented!();
    }

    pub fn get_all(&self) -> Vec<MovementType> {
        unimplemented!();
    }

    pub fn can_stay(&self) -> bool {
        unimplemented!();
    }
}

pub struct MovementBuilder;

impl MovementBuilder {
    pub fn new() -> Self {
        MovementBuilder
    }

    pub fn step(self, _dir: Direction) -> Self {
        unimplemented!();
    }

    pub fn add(self, mv: MovementType) -> Self {
        unimplemented!();
    }

    pub fn jump(self, _x: isize, _y: isize) -> Self {
        unimplemented!();
    }

    pub fn build(self) -> MovementType {
        unimplemented!();
    }

    pub fn range(self, _dir: DirectionAndRange) -> Self {
        unimplemented!();
    }

    pub fn limited_range(self, _dir: DirectionAndRange, n: usize) -> Self {
        unimplemented!();
    }

    pub fn repeat_all(self, n: usize) -> Self {
        unimplemented!();
    }

    pub fn repeat_previous(self, n: usize) -> Self {
        unimplemented!();
    }

    pub fn stay(self) -> Self {
        unimplemented!();
    }

    pub fn action_before(self, mv: MovementType, action: Box<dyn Fn(&mut Game) -> ()>) -> Self {
        unimplemented!();
    }

    pub fn action_after(self, mv: MovementType, action: Box<dyn Fn(&mut Game) -> ()>) -> Self {
        unimplemented!();
    }

    pub fn action_before_previous(self, action: Box<dyn Fn(&mut Game) -> ()>) -> Self {
        unimplemented!();
    }

    pub fn action_after_previous(self, action: Box<dyn Fn(&mut Game) -> ()>) -> Self {
        unimplemented!();
    }

    pub fn action_before_all(self, action: Box<dyn Fn(&mut Game) -> ()>) -> Self {
        unimplemented!();
    }

    pub fn action_after_all(self, action: Box<dyn Fn(&mut Game) -> ()>) -> Self {
        unimplemented!();
    }

    pub fn custom_jump(self, _dx: isize, _dy: isize, _f: Box<dyn Fn(&Board, isize, isize) -> bool>) -> Self {
        unimplemented!();
    }

    pub fn custom_range_all(self) -> Self {
        unimplemented!();
    }

    pub fn custom_range_previous(self) -> Self {
        unimplemented!();
    }

    pub fn custom(self, _f: Box<dyn Fn(&Board, usize, usize) -> (isize, isize)>) -> Self {
        unimplemented!();
    }

    pub fn add_mul(self, _v: Vec<MovementType>) -> Self {
        unimplemented!();
    }

    pub fn repeat(self, _mv: MovementType, _n: usize) -> Self {
        unimplemented!();
    }

    pub fn custom_range(self, _mv: MovementType) -> Self {
        unimplemented!();
    }

    pub fn steps(self, _dirs: Vec<Direction>) -> Self {
        unimplemented!();
    }

    pub fn only_capture(self, _mv: MovementType) -> Self {
        unimplemented!();
    }

    pub fn only_capture_previous(self) -> Self {
        unimplemented!();
    }

    pub fn only_capture_all(self) -> Self {
        unimplemented!();
    }

    pub fn capture_and_move(self, _mv: MovementType) -> Self {
        unimplemented!();
    }

    pub fn capture_and_move_previous(self) -> Self {
        unimplemented!();
    }

    pub fn capture_and_move_all(self) -> Self {
        unimplemented!();
    }

    pub fn capture_without_moving(self, _mv: MovementType) -> Self {
        unimplemented!();
    }

    pub fn capture_without_moving_previous(self) -> Self {
        unimplemented!();
    }

    pub fn capture_without_moving_all(self) -> Self {
        unimplemented!();
    }

    pub fn compose_previous(self) -> Self {
        unimplemented!();
    }

    pub fn compose_with(self, _moves: Vec<MovementType>) -> Self {
        unimplemented!();
    }

    pub fn composition(self, _moves: Vec<MovementType>) -> Self {
        unimplemented!();
    }
}
