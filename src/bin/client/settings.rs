use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Settings for the game
#[derive(Debug, Clone)]
pub struct SharmatSettings(Rc<RefCell<HashMap<String, SharmatSettingType>>>);

/// Values for sharmat settings
#[derive(Debug, Copy, Clone)]
pub enum SharmatSettingType {
    Bool(bool),
    USize(usize),
    F32(f32),
}

impl SharmatSettings {
    pub fn new(settings: HashMap<String, SharmatSettingType>) -> Self {
        Self(Rc::new(RefCell::new(settings)))
    }

    pub fn get_bool(&self, name: &str) -> Option<bool> {
        self.0.borrow().get(name).map(|v| v.as_bool()).flatten()
    }
}

impl SharmatSettingType {
    pub fn as_bool(self) -> Option<bool> {
        match self {
            SharmatSettingType::Bool(x) => Some(x),
            _ => None,
        }
    }
}
