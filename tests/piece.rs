use sharmat::piece::*;

#[test]
fn piece_create_builder() {
    let _piece = PieceBuilder::new();
}

#[test]
fn piece_create_with_id() {
    let _piece = PieceBuilder::new().id("piece_name").build();
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
    let piece = PieceBuilder::new().id("piece_name").build();
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
