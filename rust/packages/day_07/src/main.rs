use aoc;
use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::is_digit,
    error::ParseError,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
enum File {
    File(usize, String),
    Dir(String),
}

impl File {
    pub fn size(&self) -> usize {
        match self {
            File::File(size, _) => *size,
            File::Dir(_) => 0,
        }
    }
}

#[derive(Debug)]
enum Input {
    File(File),
    Cd(String),
    Ls,
}

#[derive(Debug)]
enum DirStatKind {
    File(usize),
    Dir(DirStat),
}
#[derive(Debug)]
struct DirStat {
    current_dir: String,
    slug: String,
    path: String,
    files: HashMap<String, DirStatKind>,
}

impl DirStat {
    pub fn default() -> DirStat {
        DirStat {
            current_dir: "/".to_string(),
            slug: "".to_string(),
            path: "/".to_string(),
            files: HashMap::default(),
        }
    }
    pub fn dirs(&self) -> HashMap<&str, &DirStat> {
        self.files
            .iter()
            .fold(HashMap::default(), |mut acc, (key, value)| match value {
                DirStatKind::File(_) => acc,
                DirStatKind::Dir(stat) => {
                    acc.insert(key, stat);
                    acc
                }
            })
    }
    pub fn new_relative(path: &str, slug: &str) -> Self {
        let mut next = DirStat::default();
        next.slug = slug.to_string();
        next.path = format!("{}/{}", path, slug).replace("//", "/");
        next
    }
    // case: / => self
    // case: /a => a
    // case: /a/b => b
    // given a/b/c, get the associated DirStat
    pub fn get_dirstat(&mut self, dir: &str) -> &mut Self {
        if dir == "" || dir == "/" {
            return self;
        }
        let simple_dir = if dir.starts_with("/") {
            dir.chars().skip(1).collect()
        } else {
            dir.to_owned()
        };
        let mut is_first = true;
        let (first_slugs, rest_slugs): (Vec<_>, Vec<_>) = simple_dir.split("/").partition(|slug| {
            if is_first {
                is_first = false;
                true
            } else {
                is_first
            }
        });

        match (first_slugs.first(), rest_slugs.len()) {
            (None, _) => self,
            (Some(&slug), _) => {
                match self
                    .files
                    .entry(slug.to_string())
                    .or_insert(DirStatKind::Dir(DirStat::new_relative(&self.path, slug)))
                {
                    DirStatKind::Dir(stat) => stat.get_dirstat(&rest_slugs.join("/")),
                    _ => panic!("only directory expected"),
                }
            }
        }
    }

    pub fn play_output(self: &mut Self, input: Input) {
        match input {
            Input::Cd(p) => {
                let full_dirname = format!("{}/{}", &self.current_dir, &p).replace("//", "/");
                let stat = self.get_dirstat(if p.starts_with("/") {
                    &p
                } else {
                    &full_dirname
                });
                self.current_dir = stat.path.clone();
                println!("cd {} (current: {})", &p, self.current_dir);
                ()
            }
            Input::File(File::Dir(dir)) => {
                let full_dirname = format!("{}/{}", &self.current_dir, &dir).replace("//", "/");
                let stat = self.get_dirstat(&full_dirname);
                println!("dir {} ({})", &dir, &stat.path);
            }
            Input::File(File::File(a, b)) => {
                println!("file: ({}, {})", a, &b);
                let dir = self.current_dir.clone();
                let stat = self.get_dirstat(&dir);
                stat.files.insert(b, DirStatKind::File(a));
            }
            Input::Ls => {
                // let stat = self.get_dirstat(&self.current_dir);
            }
        };
    }
    pub fn total_file_size(&self) -> usize {
        self.files
            .iter()
            .map(|(_, ftype)| match ftype {
                DirStatKind::File(size) => *size,
                DirStatKind::Dir(d) => d.total_file_size(),
            })
            .sum()
    }
    pub fn total_file_size_p1(&self) -> usize {
        let my_total = self.total_file_size();
        let contribution = if my_total <= 100000 { my_total } else { 0 };
        self.dirs()
            .iter()
            .map(|(_, &stat)| stat.total_file_size_p1())
            .sum::<usize>()
            + contribution
    }
}

fn parse_ls(i: &str) -> IResult<&str, Input> {
    let (i, _) = tag("$ ls")(i)?;
    Ok((i, Input::Ls))
}

fn parse_cd(i: &str) -> IResult<&str, Input> {
    let (i, _) = tag("$ cd ")(i)?;
    Ok((i, Input::Cd(i.to_string())))
}
fn parse_dir(i: &str) -> IResult<&str, Input> {
    let (i, _) = tag("dir ")(i)?;
    Ok((i, Input::File(File::Dir(i.to_string()))))
}

fn parse_file_bytes(i: &[u8]) -> IResult<&[u8], Input> {
    let (fname, digits) = take_while(is_digit)(i)?;
    Ok((
        i,
        Input::File(File::File(
            String::from_utf8_lossy(digits).parse::<usize>().unwrap(),
            String::from_utf8_lossy(fname).trim().to_string(),
        )),
    ))
}

fn parse_file(i: &str) -> IResult<&str, Input> {
    match parse_file_bytes(i.as_bytes()) {
        Ok((_, y)) => Ok((i, y)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::from_error_kind(
            i,
            nom::error::ErrorKind::AlphaNumeric,
        ))),
    }
}

fn parse_shell(line: &str) -> Input {
    alt((parse_cd, parse_ls, parse_dir, parse_file))(line)
        .unwrap()
        .1
}

fn main() {
    // let lines = aoc::lines("./input_p1.txt");
    let lines = aoc::lines("packages/day_07/input_p1.txt");
    let mut dir_stat = DirStat::default();
    for input in lines.iter().map(|l| parse_shell(l)) {
        dir_stat.play_output(input);
    }
    println!("p1: {}", dir_stat.total_file_size_p1());
    // println!("p2: {}", solve(&input, 14));
}

#[cfg(test)]
mod test_day_07 {
    use crate::*;

    #[test]
    fn test_demo_input_p1() {
        let lines = aoc::lines("packages/day_07/input_p1_demo.txt");
        // let lines = aoc::lines("input_p1_demo.txt");
        let mut dir_stat = DirStat::default();
        for input in lines.iter().map(|l| parse_shell(l)) {
            dir_stat.play_output(input);
        }
        assert_eq!(95437, dir_stat.total_file_size_p1());
    }
}
