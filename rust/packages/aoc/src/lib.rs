use std::{self, path::PathBuf};

pub fn lines(name: &str) -> Vec<String> {
    let pathbuf: PathBuf = [
        &std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_owned(),
        name,
    ]
    .iter()
    .collect();
    let str = pathbuf.to_string_lossy().to_string();
    if let Ok(file_str) = std::fs::read_to_string(pathbuf) {
        file_str
            .trim_end()
            .split('\n')
            .map(|x| {
                let y = String::from(x);
                y
            })
            .collect()
    } else {
        panic!("input not found: {}", &str)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
