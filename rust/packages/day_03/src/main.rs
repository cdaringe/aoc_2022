use std::collections::HashSet;

use aoc;

pub fn priority(c: char) -> usize {
    let code = c as u32;
    match code {
        65..=90 => ((code as usize) - 65) + 27,
        97..=122 => (code as usize) - 96,
        _ => panic!("invalid priority"),
    }
}

#[derive(Clone)]
struct Sack(String);

impl Sack {
    pub fn compartment_collider(&self) -> char {
        let (c1, c2) = self.0.split_at(self.0.len() / 2);
        let c1_chars: HashSet<char> = HashSet::from_iter(c1.chars());
        let c2_chars: HashSet<char> = HashSet::from_iter(c2.chars());
        let common = c1_chars.intersection(&c2_chars);
        let c = common
            .take(1)
            .next()
            .expect("couldn't find intersecting char");
        *c
    }
}

struct Group(Sack, Sack, Sack);

impl Group {
    pub fn group_item(&self) -> char {
        let c1_chars: HashSet<char> = HashSet::from_iter(self.0 .0.chars());
        let c2_chars: HashSet<char> = HashSet::from_iter(self.1 .0.chars());
        let c3_chars: HashSet<char> = HashSet::from_iter(self.2 .0.chars());
        let tmp = c1_chars.intersection(&c2_chars);
        let tmp_set: HashSet<char> = HashSet::from_iter(tmp.map(|x| *x));
        let common = tmp_set.intersection(&c3_chars);
        let c = common
            .take(1)
            .next()
            .expect("couldn't find intersecting char");
        *c
    }
}
fn parse(lines: &Vec<String>) -> Vec<Sack> {
    lines.iter().map(|v| Sack(String::to_owned(v))).collect()
}

fn main() {
    let input = aoc::lines("./input_p1.txt");
    let sacks = parse(&input);
    drop(input);
    println!(
        "p1: {}",
        sacks
            .iter()
            .map(Sack::compartment_collider)
            .map(priority)
            .sum::<usize>()
    );
    println!(
        "p2: {}",
        sacks
            .chunks(3)
            .into_iter()
            .map(|x| match x {
                [a, b, c] => Group(a.clone(), b.clone(), c.clone()),
                _ => panic!("whoops"),
            })
            .map(|x| x.group_item())
            .map(priority)
            .sum::<usize>()
    );
}

#[cfg(test)]
mod test_day_03 {
    use crate::*;

    #[test]
    fn test_demo_input_p2() {
        let input = aoc::lines("./packages/day_03/input_p1_demo.txt");
        let sacks = parse(&input);
        drop(input);
        let v = sacks
            .chunks(3)
            .into_iter()
            .map(|x| match x {
                [a, b, c] => Group(a.clone(), b.clone(), c.clone()),
                _ => panic!("whoops"),
            })
            .map(|x| x.group_item())
            .map(priority)
            .sum::<usize>();
        assert_eq!(v, 20);
    }
}
