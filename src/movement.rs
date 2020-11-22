use crate::board::{Board, BoardResult};
use crate::player::Player;
use std::fmt;

#[derive(Clone, Debug)]
pub enum MovementType {
    /// The null movement, yields by itself ∅
    Stay,
    /// Describes a piece's movements based on any orthogonal basis.
    /// `Undirected(a, b)` is equivalent to moving `a` squares in any direction and `b` in the other direction.
    ///
    /// ## Example:
    ///
    /// ```rust,ignore
    /// let knight_movement = MovementType::Undirected(2, 1);
    /// let elephant_movement = MovementType::Undirected(2, 2);
    /// let wazir_movement = MovementType::Undirected(1, 0); // could also be MovementType::Undirected(0, 1)
    /// ```
    Undirected(usize, usize),

    /// Describes a piece's unique movement on the (x, y) basis.
    /// `Directed(dx, dy)` is equivalent to moving a piece that is on `(x, y)` to `(x + dx, y + dy)`.
    ///
    /// ## Example:
    ///
    /// ```rust,ignore
    /// let white_pawn_movement = MovementType::Directed(0, 1);
    /// ```
    Directed(isize, isize),

    /// Turns regular movement types (Undirected, Directed) into a ranging movement type
    /// `RangeAny(Directed(dx, dy))` is equivalent to moving a piece that is on `(x, y)` to `(x + n*dx, y + n*dy)`, with any `n > 1`.
    /// No other piece must stand in that piece's path; any opponent's piece will be taken and the piece will stop.
    ///
    /// If MovementType::Stay is within the input movement type, then it will also be yielded (once)
    ///
    /// ## Example:
    ///
    /// ```rust,ignore
    /// let lance_movement = MovementType::RangeAny(Box::new(MovementType::Directed(0, 1)));
    /// ```
    RangeAny(Box<MovementType>),

    /// Turns regular movement types (Undirected, Directed) into a limited, ranging movement type
    /// `Range(Directed(dx, dy), max)` is equivalent to moving a piece that is on `(x, y)` to `(x + n*dx, y + n*dy)`, with any `1 < n ≤ max`.
    /// No other piece must stand in that piece's path; any opponent's piece will be taken and the piece will stop.
    ///
    /// If MovementType::Stay is within the input movement type, then it will also be yielded (once)
    ///
    /// ## Example:
    ///
    /// ```rust,ignore
    /// let double_pawn_movement = MovementType::Range(Box::new(MovementType::Directed(0, 1)), 2);
    /// ```
    Range(Box<MovementType>, usize),

    /// Assembles two or more movement types into a union of both of them
    /// `Union(a, b, c, ..., ω)` is equivalent to being able to do `a` OR `b` OR `c` OR ... OR `ω`.
    ///
    /// ## Example:
    ///
    /// ```rust,ignore
    /// let king_movement = MovementType::Union(vec![MovementType::Undirected(0, 1), MovementType::Undirected(1, 1)]);
    /// ```
    Union(Vec<MovementType>),

    /// Assembles two movement types into a so-called "hook mover" or "bent rider"
    /// This is similar to simply having a composition of two moves, except that the piece may not progress after having captured one piece
    /// `Hook(a, b)` is equivalent to `a` THEN `b`
    ///
    /// If the second move is optional, then make sure that `MovementType::Stay` is a part of it.
    /// No verification that a turn is really made is done.
    ///
    /// Note that the resulting, flattened list of possible moves will lose all sense of "bend".
    /// This means that the following example will likely produce a board filled with every tile highlighted.
    ///
    /// ## Example:
    ///
    /// ```rust,ignore
    /// let hook_mover = MovementType::Union(vec![
    ///     // vertical then horizontal
    ///     MovementType::Hook(
    ///         Box::new(MovementType::RangeAny(Box::new(MovementType::Union(vec![
    ///             MovementType::Directed(0, 1),
    ///             MovementType::Directed(0, -1),
    ///         ])))),
    ///         Box::new(MovementType::Union(vec![
    ///             MovementType::RangeAny(Box::new(MovementType::Union(vec![
    ///                 MovementType::Directed(1, 0),
    ///                 MovementType::Directed(-1, 0),
    ///             ]))),
    ///             MovementType::Stay,
    ///         ])),
    ///     ),
    ///     // horizontal then vertical
    ///     MovementType::Hook(
    ///         Box::new(MovementType::RangeAny(Box::new(MovementType::Union(vec![
    ///             MovementType::Directed(1, 0),
    ///             MovementType::Directed(-1, 0),
    ///         ])))),
    ///         Box::new(MovementType::Union(vec![
    ///             MovementType::RangeAny(Box::new(MovementType::Union(vec![
    ///                 MovementType::Directed(0, 1),
    ///                 MovementType::Directed(0, -1),
    ///             ]))),
    ///             MovementType::Stay,
    ///         ])),
    ///     ),
    /// ]);
    /// ```
    Hook(Box<MovementType>, Box<MovementType>),

    /// Adds one or more conditions to a movement type.
    /// See `MovementCondition` for more information on the different, possible conditions.
    ///
    /// ## Example:
    ///
    /// ```rust,ignore
    /// let pawn_movement = MovementType::Union(vec![
    ///     MovementType::Condition(Box::new(MovementType::Directed(0, 1)), MovementCondition::AsWhite),
    ///     MovementType::Condition(Box::new(MovementType::Directed(0, -1)), MovementCondition::AsBlack),
    /// ]);
    /// ```
    Condition(Box<MovementType>, Vec<MovementCondition>),
    // Custom?
}

pub enum MovementCondition {
    /// If the target square must be occupied by an opponent's piece
    Capture,
    /// If the target square is not occupied by any piece
    NoCapture,
    /// If the current player is white
    AsWhite,
    /// If the current player is black
    AsBlack,
    /// A custom condition
    Custom(&'static (dyn Fn(&Board, &Player, &Vec<Action>, usize, usize, isize, isize) -> bool + 'static)),
}

pub type RawMovement = (isize, isize);

impl Copy for MovementCondition {}

impl Clone for MovementCondition {
    fn clone(&self) -> Self {
        match self {
            MovementCondition::Capture => MovementCondition::Capture,
            MovementCondition::NoCapture => MovementCondition::NoCapture,
            MovementCondition::AsWhite => MovementCondition::AsWhite,
            MovementCondition::AsBlack => MovementCondition::AsBlack,
            MovementCondition::Custom(f) => MovementCondition::Custom(*f),
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
            MovementCondition::Custom(_) => write!(f, "Custom(<fn>)"),
        }
    }
}

impl MovementCondition {
    pub fn validate(
        &self,
        board: &Board,
        player: &Player,
        previous_actions: &Vec<Action>,
        x: usize,
        y: usize,
        dx: isize,
        dy: isize,
    ) -> bool {
        match self {
            MovementCondition::Capture => board
                .get((x as isize + dx) as usize, (y as isize + dy) as usize)
                .ok()
                .flatten()
                .is_some(),
            MovementCondition::NoCapture => board
                .get((x as isize + dx) as usize, (y as isize + dy) as usize)
                .ok()
                .flatten()
                .is_none(),
            MovementCondition::AsWhite => player.color.white(),
            MovementCondition::AsBlack => player.color.black(),
            MovementCondition::Custom(f) => f(board, player, previous_actions, x, y, dx, dy),
        }
    }
}

impl MovementType {
    /**
    Evaluates a MovementType's branches down into a set of possible, raw movements (dx, dy).
    **/
    pub fn flatten(
        &self,
        board: &Board,
        player: &Player,
        previous_actions: &Vec<Action>,
        x: usize,
        y: usize,
    ) -> Option<Vec<RawMovement>> {
        match self {
            MovementType::Stay => Some(vec![(0, 0)]),
            MovementType::Undirected(dx, dy) => {
                let mut res = vec![];
                let mut try_append = |dx: isize, dy: isize| {
                    if is_within_bounds(board, x as isize + dx, y as isize + dy) {
                        let target_piece = board
                            .get((x as isize + dx) as usize, (y as isize + dy) as usize)
                            .ok()
                            .flatten();
                        if target_piece.is_none() || target_piece.unwrap().1 != player.color {
                            res.push((dx, dy));
                        }
                    }
                };
                let dx = *dx as isize;
                let dy = *dy as isize;
                if dx == dy {
                    try_append(dx, dy);
                    try_append(-dx, dy);
                    try_append(dx, -dy);
                    try_append(-dx, -dy);
                } else {
                    // hard-coded permutations; idc we're in 2d
                    try_append(dx, dy);
                    try_append(-dx, dy);
                    try_append(dx, -dy);
                    try_append(-dx, -dy);
                    try_append(dy, dx);
                    try_append(-dy, dx);
                    try_append(dy, -dx);
                    try_append(-dy, -dx);
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
                let mut res: Vec<(isize, isize)> = vec![];
                for child_movement in mv.flatten(board, player, previous_actions, x, y)?.into_iter() {
                    let (dx, dy) = child_movement.clone();
                    if dx == 0 && dy == 0 {
                        if !res.iter().find(|(x, y)| *x == 0 && *y == 0).is_some() {
                            res.push((0, 0));
                        }
                        continue;
                    }
                    for mult in 1..=(board.width.get().max(board.height.get()) as isize) {
                        if is_within_bounds(board, x as isize + dx * mult, y as isize + dy * mult) {
                            let target_piece = board
                                .get(
                                    (x as isize + dx * mult) as usize,
                                    (y as isize + dy * mult) as usize,
                                )
                                .ok()
                                .flatten();
                            if target_piece.is_some() && target_piece.unwrap().1 != player.color {
                                res.push((dx * mult, dy * mult));
                                break;
                            } else if target_piece.is_some() {
                                break;
                            } else {
                                res.push((dx * mult, dy * mult));
                            }
                        } else {
                            break;
                        }
                    }
                }
                Some(res)
            }
            MovementType::Range(mv, max_range) => {
                let mut res: Vec<(isize, isize)> = vec![];
                for child_movement in mv.flatten(board, player, previous_actions, x, y)?.into_iter() {
                    let (dx, dy) = child_movement.clone();
                    if dx == 0 && dy == 0 {
                        if !res.iter().find(|(x, y)| *x == 0 && *y == 0).is_some() {
                            res.push((0, 0));
                        }
                        continue;
                    }
                    for mult in 1..=(*max_range as isize) {
                        if is_within_bounds(board, x as isize + dx * mult, y as isize + dy * mult) {
                            let target_piece = board
                                .get(
                                    (x as isize + dx * mult) as usize,
                                    (y as isize + dy * mult) as usize,
                                )
                                .ok()
                                .flatten();
                            if target_piece.is_some() && target_piece.unwrap().1 != player.color {
                                res.push((dx * mult, dy * mult));
                                break;
                            } else if target_piece.is_some() {
                                break;
                            } else {
                                res.push((dx * mult, dy * mult));
                            }
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
                    for raw_mv in mv.flatten(board, player, previous_actions, x, y)?.into_iter() {
                        res.push(raw_mv);
                    }
                }
                Some(res)
            }
            MovementType::Hook(first, second) => {
                let mut res = vec![];
                for first_mv in first.flatten(board, player, previous_actions, x, y)?.into_iter() {
                    let x2 = (x as isize + first_mv.0) as usize;
                    let y2 = (y as isize + first_mv.1) as usize;
                    let target_piece = board.get(x2, y2).ok().flatten();
                    if target_piece.is_none() {
                        for second_mv in second.flatten(board, player, previous_actions, x2, y2)?.into_iter() {
                            res.push((second_mv.0 + first_mv.0, second_mv.1 + first_mv.1));
                        }
                    } else if target_piece.unwrap().1 != player.color {
                        if second
                            .flatten(board, player, previous_actions, x2, y2)?
                            .into_iter()
                            .find(|(dx, dy)| *dx == 0 && *dy == 0)
                            .is_some()
                        {
                            res.push(first_mv.clone());
                        }
                    }
                }
                Some(res)
            }
            MovementType::Condition(mv, tags) => {
                let mut res = vec![];
                for raw_mv in mv.flatten(board, player, previous_actions, x, y)?.into_iter() {
                    if tags
                        .iter()
                        .all(|t| t.validate(board, player, previous_actions, x, y, raw_mv.0, raw_mv.1))
                    {
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

#[derive(Clone, Debug)]
pub enum Action {
    /// Doing nothing
    Stay,
    /// Moving the piece from (x, y) to (x2, y2)
    Movement(usize, usize, usize, usize),
    // TODO: promotion
}

impl Action {
    pub fn execute(&self, board: &mut Board, _player: &mut Player) -> BoardResult<()> {
        match self {
            Action::Stay => Ok(()),
            Action::Movement(x, y, x2, y2) => {
                let piece = board.get(*x, *y)?;

                board.set(*x, *y, None)?;
                board.set(*x2, *y2, piece)?;

                Ok(())
            }
        }
    }
}
