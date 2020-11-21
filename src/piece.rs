use super::movement::*;
use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Piece {
    id: String,
    alias: Vec<String>,
    display_white: usize,
    display_black: usize,
    desc: String,
    movement_type: Vec<MovementType>,
}

impl Piece {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn alias(&self) -> String {
        self.alias.join("; ")
    }

    pub fn alias_list(&self) -> &Vec<String> {
        &self.alias
    }

    pub fn display_white(&self) -> &str {
        if self.display_white == 0 || self.alias.len() < self.display_white {
            &self.id
        } else {
            &self.alias[self.display_white - 1]
        }
    }

    pub fn display_black(&self) -> &str {
        if self.display_black == 0 || self.alias.len() < self.display_black {
            &self.id
        } else {
            &self.alias[self.display_black - 1]
        }
    }

    pub fn desc(&self) -> &str {
        &self.desc
    }

    pub fn movement_type(&self) -> &Vec<MovementType> {
        &self.movement_type
    }
}

impl std::cmp::PartialEq<Piece> for Piece {
    fn eq(&self, x: &Self) -> bool {
        self.id == x.id
            && self.alias == x.alias
            && self.display_black == x.display_black
            && self.display_white == x.display_white
            && self.desc == x.desc
    }
}

pub struct PieceBuilder {
    piece_id: String,
    piece_alias: Vec<String>,
    piece_desc: String,
    piece_display_white: usize,
    piece_display_black: usize,
    piece_movement_type: Vec<MovementType>,
}

impl Default for PieceBuilder {
    fn default() -> Self {
        PieceBuilder {
            piece_id: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Couldn't get the system clock?")
                .as_millis()
                .to_string(),
            piece_alias: vec![],
            piece_desc: String::new(),
            piece_display_white: 0,
            piece_display_black: 0,
            piece_movement_type: vec![MovementType::Stay],
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

    pub fn display_white(mut self, id: &str) -> Self {
        self.piece_display_white = self.piece_alias.len() + 1;
        self.piece_alias.push(id.to_string());
        self
    }

    pub fn display_black(mut self, id: &str) -> Self {
        self.piece_display_black = self.piece_alias.len() + 1;
        self.piece_alias.push(id.to_string());
        self
    }

    pub fn desc(mut self, desc: &str) -> Self {
        if !self.piece_desc.is_empty() {
            self.piece_desc.push('\n');
        }
        self.piece_desc.push_str(desc);
        self
    }

    pub fn movement(mut self, movement_type: Vec<MovementType>) -> Self {
        self.piece_movement_type = movement_type;
        self
    }

    pub fn build(self) -> Piece {
        Piece {
            id: self.piece_id,
            alias: self.piece_alias,
            desc: self.piece_desc,
            display_black: self.piece_display_black,
            display_white: self.piece_display_white,
            movement_type: self.piece_movement_type,
        }
    }
}
