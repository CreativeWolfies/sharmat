use iced::svg::Handle;
use std::collections::HashMap;
use std::fs;

pub fn load_assets(assets_dir: String) -> HashMap<String, Handle> {
    let files = fs::read_dir(assets_dir.clone()).unwrap_or_else(|e| panic!("Couldn't read directory {}: {}", assets_dir, e)).map(|f| f.unwrap()).filter(|f| f.metadata().unwrap().is_file());
    let mut res: HashMap<String, Handle> = HashMap::new();

    for file in files {
        let svg = Handle::from_path(file.path());
        res.insert(file.path().file_stem().unwrap().to_str().unwrap().to_string(), svg);
    }

    res
}
