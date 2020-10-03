use iced::{Container, Element, Length, Svg};
use std::collections::HashMap;
use std::fs;

pub fn load_assets(assets_dir: String) -> HashMap<String, Svg> {
    let files = fs::read_dir(assets_dir.clone()).unwrap_or_else(|e| panic!("Couldn't read directory {}: {}", assets_dir, e)).map(|f| f.unwrap()).filter(|f| f.metadata().unwrap().is_file());
    let mut res: HashMap<String, Svg> = HashMap::new();

    for file in files {
        let svg = Svg::from_path(file.path());
        res.insert(file.file_name().into_string().unwrap(), svg);
    }

    res
}
