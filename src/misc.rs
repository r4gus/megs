use std::{
    path::{Path},
};

/// A point in 2d space.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn parse_path(wasm_file: &Path) -> Option<(String, String)> {
    let category = match wasm_file.parent() {
        Some(parent) => {
            match parent.file_stem() {
                Some(stem) => {
                    if let Some(stem) = stem.to_str() {
                        stem 
                    } else {
                        return None;
                    }
                },
                None => { return None; },
            }
        },
        None => { return None; },
    };
    let name = match wasm_file.file_stem() {
        Some(stem) => {
            if let Some(stem) = stem.to_str() {
                stem 
            } else {
                return None;
            }
        },
        None => { return None; },
    };

    Some((category.to_string(), name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn path_test_1() {
        let path = Path::new("/Gates/And.wasm");
        assert_eq!(parse_path(&path), Some(("Gates".to_string(), "And".to_string())));
    }

    #[test]
    fn path_test_2() {
        let path = Path::new("./assets/modules/Gates/or.wasm");
        assert_eq!(parse_path(&path), Some(("Gates".to_string(), "or".to_string())));
    }

    #[test]
    fn path_test_3() {
        let path = Path::new("And.wasm");
        assert_eq!(parse_path(&path), None);
    }
}
