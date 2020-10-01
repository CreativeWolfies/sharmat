use std::time::{SystemTime, UNIX_EPOCH};
use std::default::Default;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    piece_id: String,
    piece_alias: String,
    piece_desc: String,
}

impl Piece {
    pub fn id(&self) -> &str {
        &self.piece_id
    }

    pub fn alias(&self) -> &str {
        &self.piece_alias
    }

    pub fn desc(&self) -> &str {
        &self.piece_desc
    }
}

pub struct PieceBuilder {
    piece_id: String,
    piece_alias: String,
    piece_desc: String,
}

impl Default for PieceBuilder {
    fn default() -> Self {
        PieceBuilder {
            piece_id: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("How?")
                .as_millis()
                .to_string(),
            ..Default::default()
        }
    }
}

impl PieceBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.piece_id = id.to_string();
        self
    }

    pub fn alias(&mut self, alias: &str) -> &mut Self {
        self.piece_alias.push_str("; ");
        self.piece_alias.push_str(alias);
        self
    }

    pub fn desc(&mut self, desc: &str) -> &mut Self {
        self.piece_desc.push('\n');
        self.piece_desc.push_str(desc);
        self
    }

    pub fn build(&self) -> Piece {
        Piece {
            piece_id: self.piece_id.clone(),
            piece_alias: self.piece_alias.clone(),
            piece_desc: self.piece_desc.clone(),
        }
    }
}
