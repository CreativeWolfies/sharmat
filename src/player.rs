#[derive(Debug)]
pub struct Player {
    pub white: bool
}

impl Player {
    pub fn new(white: bool) -> Self {
        Self {
            white
        }
    }
}
