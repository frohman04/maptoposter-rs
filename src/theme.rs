use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

static THEME_DIR: &str = "themes";

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub bg: String,
    pub text: String,
    pub gradient_color: String,
    pub water: String,
    pub parks: String,
    pub road_motorway: String,
    pub road_primary: String,
    pub road_secondary: String,
    pub road_tertiary: String,
    pub road_residential: String,
    pub road_default: String,
}

impl Theme {
    /// Scan the themes directory and return a list of available theme names.
    pub fn get_available_names() -> Vec<String> {
        let path = Path::new(THEME_DIR);
        if !path.exists() {
            fs::create_dir_all(path).unwrap();
            Vec::new()
        } else {
            fs::read_dir(path)
                .unwrap()
                .filter_map(|res| match res {
                    Ok(item) => match item.file_name().into_string() {
                        Ok(filename) => {
                            if filename.ends_with(".json") {
                                Some(filename.strip_suffix(".json").unwrap().to_string())
                            } else {
                                None
                            }
                        }
                        Err(_) => None,
                    },
                    Err(_) => None,
                })
                .collect()
        }
    }

    /// Get a Theme struct by its name.
    pub fn get_by_name(name: &str) -> serde_json::error::Result<Theme> {
        let path: PathBuf = [THEME_DIR, format!("{}.json", name).as_str()]
            .iter()
            .collect();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        serde_json::from_reader(reader)
    }
}
