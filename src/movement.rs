use super::board::Board;
use super::player::Player;
use std::fmt;

#[derive(Clone, Debug)]
pub enum MovementType {
    /// The null movement, yields by itself ∅
    Stay,
    /// Describe a piece's movements based on any orthogonal basis.
    /// `Undirected(a, b)` is equivalent to moving `a` squares in any direction and `b` in the other direction.
    ///
    /// ## Example:
    ///
    /// ```rust,ignore
    /// let knight_movement = Undirected(2, 1);
    /// let elephant_movement = Undirected(2, 2);
    /// let wazir_movement = Undirected(1, 0); // could also be Undirected(0, 1)
    /// ```
    Undirected(usize, usize),
    Directed(isize, isize), // <a, b> -mover
    RangeAny(Box<MovementType>), // x-∞-rider; may not contain any Composition: make it not recursive?
    Range(Box<MovementType>, usize), // x-n-rider; may not contain any Composition: make it not recursive?
    Union(Vec<MovementType>), // a OR b OR c ... OR ω
    Condition(Box<MovementType>, Vec<MovementCondition>)
    // Custom?
}

pub enum MovementCondition {
    Capture,
    NoCapture,
    AsWhite,
    AsBlack,
    Condition(&'static (dyn Fn(&Board, &Player, usize, usize, isize, isize) -> bool + 'static))
}

type RawMovement = (isize, isize);

impl Copy for MovementCondition {}

impl Clone for MovementCondition {
    fn clone(&self) -> Self {
        match self {
            MovementCondition::Capture => MovementCondition::Capture,
            MovementCondition::NoCapture => MovementCondition::NoCapture,
            MovementCondition::AsWhite => MovementCondition::AsWhite,
            MovementCondition::AsBlack => MovementCondition::AsBlack,
            MovementCondition::Condition(f) => MovementCondition::Condition(*f),
        }
    }
}

impl fmt::Debug for MovementCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MovementCondition::Capture => write!(f, "Capture"),
            MovementCondition::NoCapture => write!(f, "NoCapture"),
            MovementCondition::AsWhite => write!(f, "AsWhite"),
            MovementCondition::AsBlack => write!(f, "AsBlack"),
            MovementCondition::Condition(_) => write!(f, "Condition(<fn>)"),
        }
    }
}

impl MovementCondition {
    fn validate(&self, board: &Board, player: &Player, x: usize, y: usize, dx: isize, dy: isize) -> bool {
        match self {
            MovementCondition::Capture => board.get((x as isize + dx) as usize, (y as isize + dy) as usize).ok().flatten().is_some(),
            MovementCondition::NoCapture => board.get((x as isize + dx) as usize, (y as isize + dy) as usize).ok().flatten().is_none(),
            MovementCondition::AsWhite => player.white,
            MovementCondition::AsBlack => !player.white,
            MovementCondition::Condition(f) => f(board, player, x, y, dx, dy),
        }
    }
}

impl MovementType {
    pub fn flatten(&self, board: &Board, player: &Player, x: usize, y: usize) -> Option<Vec<RawMovement>> {
        match self {
            MovementType::Stay => Some(vec![]),
            MovementType::Undirected(dx, dy) => {
                let mut res = vec![];
                let mut try_append = |dx: isize, dy: isize| {
                    if is_within_bounds(board, x as isize + dx, y as isize + dy) {
                        res.push((dx, dy));
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
                    Some(vec![(*dx, *dy)])
                } else {
                    Some(vec![])
                }
            }
            MovementType::RangeAny(mv) => {
                let mut res = vec![];
                for child_movement in mv.flatten(board, player, x, y)?.into_iter() {
                    let (dx, dy) = child_movement.clone();
                    for mult in 1..=(board.width.get().max(board.height.get()) as isize) {
                        if is_within_bounds(board, x as isize + dx * mult, y as isize + dy * mult) {
                            res.push((dx * mult, dy * mult));
                        } else {
                            break;
                        }
                    }
                }
                Some(res)
            }
            MovementType::Range(mv, max_range) => {
                let mut res = vec![];
                for child_movement in mv.flatten(board, player, x, y)?.into_iter() {
                    let (dx, dy) = child_movement.clone();
                    for mult in 1..=(*max_range as isize) {
                        if is_within_bounds(board, x as isize + dx * mult, y as isize + dy * mult) {
                            res.push((dx * mult, dy * mult));
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
                    for raw_mv in mv.flatten(board, player, x, y)?.into_iter() {
                        res.push(raw_mv);
                    }
                }
                Some(res)
            }
            MovementType::Condition(mv, tags) => {
                let mut res = vec![];
                for raw_mv in mv.flatten(board, player, x, y)?.into_iter() {
                    if tags.iter().all(|t| t.validate(board, player, x, y, raw_mv.0, raw_mv.1)) {
                        res.push(raw_mv);
                    }
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
