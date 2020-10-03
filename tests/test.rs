// The ~tests~ (the thing that tests functions & stuff)

use std::num::NonZeroUsize;
use sharmat::board::*;
use sharmat::piece::*;
use sharmat::rule::*;
use sharmat::game::*;
use sharmat::movement::*;

// Engine basic tests

#[cfg(test)]
mod board {
    use super::*;

    #[test]
    fn board_create() {
        let _board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    }

    #[test]
    fn board_set_name() {
        let mut board = Board::new(NonZeroUsize::new(1).unwrap(), NonZeroUsize::new(1).unwrap());
        board.set_name("Hello");
    }

    #[test]
    fn board_get_name() {
        let mut board = Board::new(NonZeroUsize::new(1).unwrap(), NonZeroUsize::new(1).unwrap());
        board.set_name("Hello");
        assert_eq!(board.name(), "Hello");
    }

    #[test]
    #[allow(unused_must_use)]
    fn board_set_piece() {
        let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
        let piece = PieceBuilder::new().build();
        board.set(0, 0, &piece);
    }

    #[test]
    fn board_oob_set_piece() {
        let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
        let piece = PieceBuilder::new().build();
        assert_eq!(board.set(20, 20, &piece).unwrap_err(), BoardError::OutOfBounds);
    }

    #[test]
    #[allow(unused_must_use)]
    fn board_get_piece() {
        let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
        let piece = PieceBuilder::new().build();
        board.set(0, 0, &piece);
        assert_eq!(board.get(0, 0).unwrap().map(|x| x.clone()), Some(piece));
    }

    #[test]
    fn board_oob_get_piece() {
        let board = Board::new(NonZeroUsize::new(3).unwrap(), NonZeroUsize::new(3).unwrap());
        assert_eq!(board.get(5, 5).unwrap_err(), BoardError::OutOfBounds);
    }

    #[test]
    #[allow(unused_must_use)]
    fn board_move_piece() {
        let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
        let piece = PieceBuilder::new().build();
        board.set(0, 0, &piece);
        board.step_piece(0, 0, 3, 3);
        assert_eq!(board.get(3, 3).unwrap().map(|x| x.clone()), Some(piece));
    }

    #[test]
    fn board_oob_move_piece_first_pos() {
        let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        assert_eq!(board.step_piece(6, 6, 0, 0).unwrap_err(), BoardError::OutOfBounds);
    }

    #[test]
    fn board_oob_move_piece_scnd_pos() {
        let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        assert_eq!(board.step_piece(0, 0, 6, 6).unwrap_err(), BoardError::OutOfBounds);
    }

    #[test]
    #[allow(unused_must_use)]
    fn board_clear_piece() {
        let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
        let piece = PieceBuilder::new().id("d").build();
        board.set(0, 0, &piece);
        board.clear_pos(0, 0);
        assert_eq!(board.get(0, 0).unwrap(), None);
    }

    #[test]
    fn board_oob_clear_piece() {
        let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        assert_eq!(board.clear_pos(6, 6).unwrap_err(), BoardError::OutOfBounds);
    }

    #[test]
    #[allow(unused_must_use)]
    fn board_clear_board() {
        let empty_board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let piece = PieceBuilder::new().build();
        board.set(0, 0, &piece);
        board.set(0, 3, &piece);
        board.set(3, 0, &piece);
        board.clear();
        assert_eq!(board, empty_board);
    }
}

#[cfg(test)]
mod piece {
    use super::*;

    #[test]
    fn piece_create_builder() {
        let _piece = PieceBuilder::new();
    }

    #[test]
    fn piece_create_with_id() {
        let _piece = PieceBuilder::new()
            .id("piece_name")
            .build();
    }

    #[test]
    fn piece_create_with_alias() {
        let _piece = PieceBuilder::new()
            .id("piece_name")
            .alias("Piece name")
            .build();
    }

    #[test]
    fn piece_create_with_description() {
        let _piece = PieceBuilder::new()
            .id("piece_name")
            .desc("It's just a test piece")
            .build();
    }

    #[test]
    fn piece_get_id() {
        let piece = PieceBuilder::new()
            .id("piece_name")
            .build();
        assert_eq!(piece.id(), "piece_name");
    }

    #[test]
    fn piece_get_alias() {
        let piece = PieceBuilder::new()
            .id("piece_name")
            .alias("Piece name")
            .build();
        assert_eq!(piece.alias(), "Piece name");
    }

    #[test]
    fn piece_get_description() {
        let piece = PieceBuilder::new()
            .id("piece_name")
            .desc("It's just a test piece")
            .build();
        assert_eq!(piece.desc(), "It's just a test piece");
    }

    #[test]
    fn piece_id_override() {
        let piece = PieceBuilder::new()
            .id("piece_name")
            .id("other_piece_name")
            .build();
        assert_eq!(piece.id(), "other_piece_name");
    }

    #[test]
    fn piece_alias_append() {
        let piece = PieceBuilder::new()
            .id("piece_name")
            .alias("Insert text")
            .alias("So-called test piece")
            .build();
        assert_eq!(piece.alias(), "Insert text; So-called test piece");
    }

    #[test]
    fn piece_description_append() {
        let piece = PieceBuilder::new()
            .id("piece_name")
            .desc("First line")
            .desc("Second line")
            .build();
        assert_eq!(piece.desc(), "First line\nSecond line");
    }
}

#[cfg(test)]
mod movement {
    use super::*;

    #[test]
    fn movement_create() {
        let _movement = MovementBuilder::new();
    }

    #[test]
    fn movement_create_with_move_enum() {
        let _movement = MovementBuilder::new()
            .add(MovementType::Step(Direction::Up))
            .build();
    }

    #[test]
    fn movement_create_with_move_enum_push() {
        let _movement = MovementBuilder::new()
            .add(MovementType::Step(Direction::Up))
            .add(MovementType::Step(Direction::Down))
            .build();
    }

    #[test]
    fn movement_create_with_moves_enum() {
        let _movement = MovementBuilder::new()
            .add_mul(vec![MovementType::Step(Direction::Up), MovementType::Step(Direction::Down)])
            .build();
    }

    #[test]
    fn movement_create_with_moves_and_move_enum() {
        let _movement = MovementBuilder::new()
            .add_mul(vec![MovementType::Step(Direction::Up), MovementType::Step(Direction::Down)])
            .add(MovementType::Step(Direction::UpRight))
            .build();
    }

    #[test]
    fn movement_create_with_move() {
        let _movement = MovementBuilder::new()
            .step(Direction::Up)
            .build();
    }

    #[test]
    fn movement_create_with_move_push() {
        let _movement = MovementBuilder::new()
            .step(Direction::Up)
            .step(Direction::Down)
            .build();
    }

    #[test]
    fn movement_create_with_moves() {
        let _movement = MovementBuilder::new()
            .steps(vec![Direction::Up, Direction::Down])
            .build();
    }

    #[test]
    fn movement_get_moves_with_move_enum() {
        let movement = MovementBuilder::new()
            .add(MovementType::Step(Direction::Up))
            .build();
        assert_eq!(movement.get_all(), vec![MovementType::Step(Direction::Up)]);
    }

    #[test]
    fn movement_get_moves_with_move_enum_push() {
        let movement = MovementBuilder::new()
            .add(MovementType::Step(Direction::Up))
            .add(MovementType::Step(Direction::Down))
            .build();
        assert_eq!(movement.get_all(), vec![MovementType::Step(Direction::Up), MovementType::Step(Direction::Down)]);
    }

    #[test]
    fn movement_get_moves_with_moves_enum() {
        let movement = MovementBuilder::new()
            .add_mul(vec![MovementType::Step(Direction::Up), MovementType::Step(Direction::Down)])
            .build();
        assert_eq!(movement.get_all(), vec![MovementType::Step(Direction::Up), MovementType::Step(Direction::Down)]);
    }

    #[test]
    fn movement_get_moves_with_moves_and_move_enum() {
        let movement = MovementBuilder::new()
            .add_mul(vec![MovementType::Step(Direction::Up), MovementType::Step(Direction::Down)])
            .add(MovementType::Step(Direction::UpRight))
            .build();
        assert_eq!(movement.get_all(), vec![MovementType::Step(Direction::Up), MovementType::Step(Direction::Down), MovementType::Step(1, 1)]);
    }

    #[test]
    fn movement_get_moves_with_move() {
        let movement = MovementBuilder::new()
            .step(Direction::Up)
            .build();
        assert_eq!(movement.get_all(), vec![MovementType::Step(Direction::Up)]);
    }

    #[test]
    fn movement_get_moves_with_move_push() {
        let movement = MovementBuilder::new()
            .step(Direction::Up)
            .step(Direction::Down)
            .build();
        assert_eq!(movement.get_all(), vec![MovementType::Step(Direction::Up), MovementType::Step(Direction::Down)]);
    }

    #[test]
    fn movement_get_moves_with_moves() {
        let movement = MovementBuilder::new()
            .steps(vec![Direction::Up, Direction::Down])
            .build();
        assert_eq!(movement.get_all(), vec![MovementType::Step(Direction::Up), MovementType::Step(Direction::Down)]);
    }

    #[test]
    fn movement_create_with_basic_moves_enum() {
        let movement = MovementBuilder::new()
            .add(MovementType::Stay)
            .add(MovementType::Step(Direction::Up))
            .add(MovementType::Jump(1, 2))
            .add(MovementType::Jump(-1, -2))
            .add(MovementType::Range(DirectionAndRange::Direction(Direction::Up)))
            .add(MovementType::Range(DirectionAndRange::Range(Range::Diagonal)))
            .add(MovementType::Range(DirectionAndRange::Range(Range::Straight)))
            .add(MovementType::LimitRange(DirectionAndRange::Direction(Direction::Down), 3))
            .build();
        assert_eq!(movement.get_all(), vec![
            MovementType::Stay,
            MovementType::Step(Direction::Up),
            MovementType::Jump(1, 2),
            MovementType::Jump(-1, -2),
            MovementType::Range(DirectionAndRange::Direction(Direction::Up)),
            MovementType::Range(DirectionAndRange::Range(Range::Diagonal)),
            MovementType::Range(DirectionAndRange::Range(Range::Straight)),
            MovementType::LimitRange(DirectionAndRange::Direction(Direction::Down), 3)
        ]);
    }

    #[test]
    fn movement_create_with_basic_moves() {
        let movement = MovementBuilder::new()
            .stay()
            .step(Direction::Up)
            .jump(1, 2)
            .jump(-1, -2)
            .range(DirectionAndRange::Direction(Direction::Up))
            .range(DirectionAndRange::Direction(Range::Diagonal))
            .range(DirectionAndRange::Direction(Range::Straight))
            .limit_range(DirectionAndRange::Direction(Direction::Down), 3)
            .build();
        assert_eq!(movement.get_all(), vec![
            MovementType::Stay,
            MovementType::Step(Direction::Up),
            MovementType::Jump(1, 2),
            MovementType::Jump(-1, -2),
            MovementType::Range(DirectionAndRange::Direction(Direction::Up)),
            MovementType::Range(DirectionAndRange::Range(Range::Diagonal)),
            MovementType::Range(DirectionAndRange::Range(Range::Straight)),
            MovementType::LimitRange(DirectionAndRange::Direction(Direction::Down), 3)
        ]);
    }

    #[test]
    fn movement_create_with_recursive_moves_enum() {
        let movement = MovementBuilder::new()
            .add(MovementType::Repeat(MovementType::Jump(1, 2), 3))
            .add(MovementType::CustomRange(MovementType::Jump(0, 3)))
            .add(MovementType::Composition(vec![MovementType::Jump(2, 2), MovementType::Range(DirectionAndRange::Range(Range::Straight))]))
            .add(MovementType::ConditionalMove(MovementType::Jump(0, 3), Box::new(|piece, _| piece.id() == "silver_general")))
            .add(MovementType::OnlyCapture(MovementType::Jump(-2, 2)))
            .add(MovementType::CaptureAndMove(MovementType::Jump(2, 2)))
            .add(MovementType::CaptureWithoutMoving(MovementType::Jump(3, 3)))
            .build();
        assert_eq!(movement.get_all(), vec![
            MovementType::Repeat(MovementType::Jump(1, 2), 3),
            MovementType::CustomRange(MovementType::Jump(0, 3)),
            MovementType::Composition(vec![MovementType::Jump(2, 2), MovementType::Range(DirectionAndRange::Range(Range::Straight))]),
            MovementType::ConditionalMove(MovementType::Jump(0, 3), Box::new(|piece, _| piece.id() == "silver_general")),
            MovementType::OnlyCapture(MovementType::Jump(-2, 2)),
            MovementType::CaptureAndMove(MovementType::Jump(2, 2)),
            MovementType::CaptureWithoutMoving(MovementType::Jump(3, 3))
        ]);
    }

    #[test]
    fn movement_create_with_recursive_moves_simple() {
        let movement = MovementBuilder::new()
            .repeat(MovementType::Jump(1, 2), 3)
            .custom_range(MovementType::Jump(0, 3))
            .composition(vec![MovementType::Jump(2, 2), MovementType::Range(DirectionAndRange::Range(Range::Straight))])
            .conditional_move(MovementType::Jump(0, 3), Box::new(|piece, _| piece.id() == "silver_general"))
            .only_capture(MovementType::Jump(-2, 2))
            .capture_and_move(MovementType::Jump(2, 2))
            .capture_without_moving(MovementType::Jump(3, 3))
            .build();
        assert_eq!(movement.get_all(), vec![
            MovementType::Repeat(MovementType::Jump(1, 2), 3),
            MovementType::CustomRange(MovementType::Jump(0, 3)),
            MovementType::Composition(vec![MovementType::Jump(2, 2), MovementType::Range(DirectionAndRange::Range(Range::Straight))]),
            MovementType::ConditionalMove(MovementType::Jump(0, 3), Box::new(|piece, _| piece.id() == "silver_general")),
            MovementType::OnlyCapture(MovementType::Jump(-2, 2)),
            MovementType::CaptureAndMove(MovementType::Jump(2, 2)),
            MovementType::CaptureWithoutMoving(MovementType::Jump(3, 3))
        ]);
    }

    #[test]
    fn movement_create_with_recursive_moves_previous() {
        let movement = MovementBuilder::new()
            .jump(1, 2)
            .repeat_previous(3)
            .jump(0, 3)
            .custom_range_previous()
            .jump(2, 2)
            .range(DirectionAndRange::Range(Range::Straight))
            .compose_previous()
            .jump(0, 3)
            .conditional_move_previous(Box::new(|piece, _| piece.id() == "silver_general"))
            .jump(-2, 2)
            .only_capture_previous()
            .jump(2, 2)
            .capture_and_move_previous()
            .jump(3, 3)
            .capture_without_moving_previous()
            .build();
        assert_eq!(movement.get_all(), vec![
            MovementType::Repeat(MovementType::Jump(1, 2), 3),
            MovementType::CustomRange(MovementType::Jump(0, 3)),
            MovementType::Composition(vec![MovementType::Jump(2, 2), MovementType::Range(DirectionAndRange::Range(Range::Straight))]),
            MovementType::ConditionalMove(MovementType::Jump(0, 3), Box::new(|piece, _| piece.id() == "silver_general")),
            MovementType::OnlyCapture(MovementType::Jump(-2, 2)),
            MovementType::CaptureAndMove(MovementType::Jump(2, 2)),
            MovementType::CaptureWithoutMoving(MovementType::Jump(3, 3))
        ]);
    }

    #[test]
    fn movement_create_with_recursive_moves_all() {
        let movement = MovementBuilder::new()
            .jump(2, 2)
            .repeat(3)
            .custom_range()
            .compose_with(vec![MovementType::Jump(3, -1)])
            .conditional_move(Box::new(|piece, _| piece.id() == "silver_general"))
            .capture_without_moving()
            .build();
        assert_eq!(movement.get_all(), vec![MovementType::CaptureWithoutMoving(
            MovementType::ConditionalMove(
                MoveMovementType::Composition(
                    vec![
                        MovementType::CustomRange(
                            MovementType::Repeat(
                                MovementType::Jump(2, 2),
                                3
                            )
                        ),
                        MovementType::Jump(3, -1)
                    ]
                ),
                Box::new(|piece, _| piece.id() == "silver_general")
            )
        )]);
    }

    /*
    -- legend --
    rx = relative x
    ry = relative y
    x = absolute x
    y = absolute y
    n = number (natural integer mostly)
    -- basic --
    Stay // does not mean the piece can stay, just says that staying is a move with this piece
    Step(Direction)
    Jump(rx, ry)
    Range(Vec<DirectionAndRange>), // A HashSet would be more convenient, but harder to create without macros from other packages.
    LimitRange(Vec<DirectionAndRange>, n)
    -- recursive --
    Repeat(MovementType, n)
    CustomRange(MovementType)
    Composition(Vec<MovementType>)
    ConditionalMove(MovementType, Box<dyn Fn(&Piece, &Player) -> bool>)
    OnlyCapture(MovementType)
    CaptureAndMove(MovementType)
    CaptureWithoutMoving(MovementType)
    -- actions --
    ActionBefore(MovementType, Box<dyn Fn(&mut Game) -> ()>)
    ActionAfter(MovementType, Box<dyn Fn(&mut Game) -> ()>)
    -- custom --
    CustomJump(rx, ry, Box<dyn Fn(&Board, x, y) -> bool>)
    Custom(Box<dyn Fn(&Board) -> (x, y)>)
    -- note --
    the recursive types can be added to all other moves in the builder at the same time (like .move(...).move(...).repeat_previous(n))
    - Transform Jump(0, 0) in stay, etc
    - Change Board::new(NonZeroUsize, NonZeroUsize) -> Self to Board::new(usize, usize) -> BoardResult<Self>
    - create tests to get specific move types
    */
}

#[cfg(test)]
mod player {
    use super::*;

    #[test]
    fn player_create() {
        let _player = Player::new(/* UNKNOWN */);
    }
}

#[cfg(test)]
mod rule {
    use super::*;

    #[test]
    fn rule_create() {
        let _rule = Rule::new(/* UNKNOWN */);
    }
}

#[cfg(test)]
mod game {
    use super::*;

    #[test]
    fn game_create() {
        let _game = GameBuilder::new();
    }

    #[test]
    fn game_create_with_board() {
        let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let _game = GameBuilder::new()
            .board(&board)
            .build();
    }

    #[test]
    fn game_create_with_board_push() {
        let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let _game = GameBuilder::new()
            .board(&board)
            .board(&board2)
            .build();
    }

    #[test]
    fn game_create_with_boards_push() {
        let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let _game = GameBuilder::new()
            .boards(vec![&board, &board2])
            .build();
    }

    #[test]
    fn game_create_with_board_and_boards() {
        let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let board3 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let _game = GameBuilder::new()
            .board(&board)
            .boards(vec![&board2, &board3])
            .build();
    }

    #[test]
    fn game_get_boards_with_board() {
        let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let game = GameBuilder::new()
            .board(&board)
            .build();
        assert_eq!(game.boards(), &vec![&board]);
    }

    #[test]
    fn game_get_boards_with_board_push() {
        let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let game = GameBuilder::new()
            .board(&board)
            .board(&board2)
            .build();
        assert_eq!(*game.boards(), vec![&board, &board2]);
    }

    #[test]
    fn game_get_boards_with_boards_push() {
        let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let game = GameBuilder::new()
            .boards(vec![&board, &board2])
            .build();
        assert_eq!(*game.boards(), vec![&board, &board2]);
    }

    #[test]
    fn game_get_boards_with_board_and_boards_push() {
        let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let board3 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
        let game = GameBuilder::new()
            .board(&board)
            .boards(vec![&board2, &board3])
            .build();
        assert_eq!(*game.boards(), vec![&board, &board2, &board3]);
    }

    #[test]
    fn game_create_with_piece() {
        let piece = PieceBuilder::new().build();
        let _game = GameBuilder::new()
            .piece(&piece)
            .build();
    }

    #[test]
    fn game_create_with_piece_push() {
        let piece = PieceBuilder::new().build();
        let piece2 = PieceBuilder::new().build();
        let _game = GameBuilder::new()
            .piece(&piece)
            .piece(&piece2)
            .build();
    }

    #[test]
    fn game_create_with_pieces_push() {
        let piece = PieceBuilder::new().build();
        let piece2 = PieceBuilder::new().build();
        let _game = GameBuilder::new()
            .pieces(vec![&piece, &piece2])
            .build();
    }

    #[test]
    fn game_create_with_piece_and_pieces_push() {
        let piece = PieceBuilder::new().build();
        let piece2 = PieceBuilder::new().build();
        let piece3 = PieceBuilder::new().build();
        let _game = GameBuilder::new()
            .piece(&piece)
            .pieces(vec![&piece2, &piece3])
            .build();
    }

    #[test]
    fn game_get_pieces_with_piece() {
        let piece = PieceBuilder::new().build();
        let game = GameBuilder::new()
            .piece(&piece)
            .build();
        assert_eq!(*game.pieces(), vec![&piece]);
    }

    #[test]
    fn game_get_pieces_with_piece_push() {
        let piece = PieceBuilder::new().build();
        let piece2 = PieceBuilder::new().build();
        let game = GameBuilder::new()
            .piece(&piece)
            .piece(&piece2)
            .build();
        assert_eq!(*game.pieces(), vec![&piece, &piece2]);
    }

    #[test]
    fn game_get_pieces_with_pieces_push() {
        let piece = PieceBuilder::new().build();
        let piece2 = PieceBuilder::new().build();
        let game = GameBuilder::new()
            .pieces(vec![&piece, &piece2])
            .build();
        assert_eq!(*game.pieces(), vec![&piece, &piece2]);
    }

    #[test]
    fn game_get_pieces_with_piece_and_pieces_push() {
        let piece = PieceBuilder::new().build();
        let piece2 = PieceBuilder::new().build();
        let piece3 = PieceBuilder::new().build();
        let game = GameBuilder::new()
            .piece(&piece)
            .pieces(vec![&piece2, &piece3])
            .build();
        assert_eq!(*game.pieces(), vec![&piece, &piece2, &piece3]);
    }

    #[test]
    fn game_search_piece_by_id_success() {
        let piece = PieceBuilder::new().build();
        let piece2 = PieceBuilder::new().build();
        let piece3 = PieceBuilder::new().build();
        let game = GameBuilder::new()
            .pieces(vec![&piece, &piece2, &piece3])
            .build();
        assert_eq!(game.search_piece(piece.id()), Some(&&piece));
    }

    #[test]
    fn game_search_piece_by_id_fail() {
        let piece = PieceBuilder::new().build();
        let piece2 = PieceBuilder::new().build();
        let piece3 = PieceBuilder::new().build();
        let game = GameBuilder::new()
            .pieces(vec![&piece, &piece2, &piece3])
            .build();
        assert_eq!(game.search_piece("random"), None);
    }

    #[test]
    fn game_search_piece_by_alias_success() {
        let piece = PieceBuilder::new().alias("test").build();
        let piece2 = PieceBuilder::new().alias("foo").alias("test").build();
        let piece3 = PieceBuilder::new().build();
        let game = GameBuilder::new()
            .pieces(vec![&piece, &piece2, &piece3])
            .build();
        assert_eq!(game.search_piece_alias("test"), vec![&&piece, &&piece2]);
    }

    #[test]
    fn game_search_piece_by_alias_fail() {
        let piece = PieceBuilder::new().alias("test").build();
        let piece2 = PieceBuilder::new().alias("foo").alias("test").build();
        let piece3 = PieceBuilder::new().build();
        let game = GameBuilder::new()
            .pieces(vec![&piece, &piece2, &piece3])
            .build();
        assert_eq!(game.search_piece_alias("hmm"), Vec::<&&Piece>::new());
    }
}
