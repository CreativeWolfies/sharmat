use std::time::{SystemTime, UNIX_EPOCH};
use std::default::Default;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    piece_id: String,
    piece_alias: Vec<String>,
    piece_desc: String,
}

impl Piece {
    pub fn id(&self) -> &str {
        &self.piece_id
    }

    pub fn alias(&self) -> String {
        self.piece_alias.join("; ")
    }

    pub fn alias_list(&self) -> &Vec<String> {
        &self.piece_alias
    }

    pub fn desc(&self) -> &str {
        &self.piece_desc
    }
}

pub struct PieceBuilder {
    piece_id: String,
    piece_alias: Vec<String>,
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
            piece_alias: vec![],
            piece_desc: String::new(),
        }
    }
}

impl PieceBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: &str) -> Self {
        self.piece_id = id.to_string();
        self
    }

    pub fn alias(mut self, alias: &str) -> Self {
        self.piece_alias.push(alias.to_string());
        self
    }

    pub fn desc(mut self, desc: &str) -> Self {
        if !self.piece_desc.is_empty() {
            self.piece_desc.push('\n');
        }
        self.piece_desc.push_str(desc);
        self
    }

    pub fn build(self) -> Piece {
        Piece {
            piece_id: self.piece_id,
            piece_alias: self.piece_alias,
            piece_desc: self.piece_desc,
        }
    }
}
