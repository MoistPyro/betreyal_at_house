use json::{self, JsonValue};
use std::{fs, path::Path, io::Error, ffi::OsStr};

fn read_json(path: &Path) -> JsonValue {
    let data = fs::read_to_string(path).expect("failed reading file.");
    json::parse(data.as_str()).expect("not a str")
}

pub fn get_jsons_in_dir(path: &str) -> Result<Vec<JsonValue>, Error> {
    let path: &Path = Path::new(path);
    let mut r: Vec<JsonValue> = Vec::<JsonValue>::new();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().extension() == Some(OsStr::new("json")) {
                r.push(read_json(entry.path().as_path()));
            }
        }
    }
    Ok(r)
}