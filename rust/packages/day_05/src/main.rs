// use std::collections::HashSet;

use aoc;

#[derive(Clone)]
struct CrateZone(Vec<Vec<char>>);

impl CrateZone {
    pub fn apply_bulk_move(&mut self, mv: &(usize, usize, usize)) -> () {
        println!("{:?}", mv);
        let src = mv.1 - 1;
        let dest = mv.2 - 1;
        let mut to_move: Vec<char> = vec![];
        let mut i = 1;
        while i <= mv.0 {
            to_move.push(self.0[src].pop().expect("char missing from stack"));
            i = i + 1;
        }
        to_move.reverse();
        self.0[dest].append(&mut to_move);
    }

    pub fn apply_move(&mut self, mv: &(usize, usize, usize)) -> () {
        println!("{:?}", mv);
        let src = mv.1 - 1;
        let dest = mv.2 - 1;
        let mut i = 1;
        while i <= mv.0 {
            let popped = self.0[src].pop().expect("char missing from stack");
            self.0[dest].push(popped);
            i = i + 1;
        }
    }
    pub fn from_lines(lines: &[String]) -> Self {
        let char_rows: Vec<Vec<char>> = lines
            .iter()
            .rev()
            .skip(1)
            .map(|l| {
                l.chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|chunk| {
                        let c = chunk[1];
                        c
                    })
                    .collect::<Vec<char>>()
            })
            .collect();
        let mut cols: Vec<Vec<char>> = (1..=9)
            .map(|_| (vec![] as Vec<char>))
            .collect::<Vec<Vec<char>>>();
        for row in char_rows {
            for (col_i, char) in row.iter().enumerate() {
                if *char != ' ' {
                    cols[col_i].push(*char)
                }
            }
        }
        CrateZone(cols)
    }
}

fn parse_moves(lines: &[String]) -> Vec<(usize, usize, usize)> {
    lines
        .iter()
        .map(|l| {
            let mut digits = l
                .split_whitespace()
                .filter_map(|s| match s.parse::<usize>() {
                    Ok(v) => Some(v),
                    _ => None,
                });
            let a = digits.next().expect("move digit missing");
            let b = digits.next().expect("move digit missing");
            let c = digits.next().expect("move digit missing");
            let r: (usize, usize, usize) = (
                (a as usize) as usize,
                (b as usize) as usize,
                (c as usize) as usize,
            );
            r
        })
        .collect::<Vec<(usize, usize, usize)>>()
}

fn main() {
    // let input = aoc::lines("./packages/day_05/input_p1.txt");
    let input = aoc::lines("./input_p1.txt");
    let mut input_iter = input.split(|l| {
        let is_empty = l == "";
        is_empty
    });
    let crate_lines = input_iter.next().unwrap();
    let mut cratezone = CrateZone::from_lines(crate_lines);
    let moves = parse_moves(&input_iter.next().unwrap());

    // p1
    // moves.iter().for_each(|mv| cratezone.apply_move(mv));
    // let p1 = String::from_iter(cratezone.0.iter().filter_map(|col| col.last()));
    // println!("p1: {}", p1);

    // p2
    moves.iter().for_each(|mv| cratezone.apply_bulk_move(mv));
    let p2 = String::from_iter(cratezone.0.iter().filter_map(|col| col.last()));
    println!("p2: {}", p2);
}

#[cfg(test)]
mod test_day_05 {
    use crate::*;

    #[test]
    fn test_demo_input_p1() {
        let input = aoc::lines("./input_p1_demo.txt");
        // let input = aoc::lines("./packages/day_05/input_p1_demo.txt");
        let mut input_iter = input.split(|l| {
            let is_empty = l == "";
            is_empty
        });
        let crate_lines = input_iter.next().unwrap();
        let mut cratezone = CrateZone::from_lines(crate_lines);
        let moves = parse_moves(&input_iter.next().unwrap());
        moves.iter().for_each(|mv| cratezone.apply_move(mv));
        let top_str = String::from_iter(cratezone.0.iter().filter_map(|col| col.last()));
        assert_eq!(&top_str, "CMZ");
    }
}
