use std::collections::HashSet;

use aoc;

#[derive(Clone)]
struct ElfCleanPair(HashSet<u8>, HashSet<u8>);

impl ElfCleanPair {
    pub fn from_line(line: &String) -> Self {
        let mut it = line
            .split(',')
            .map(|x| x.split('-'))
            .map(|mut it| {
                let start = it.next().unwrap();
                let end = it.next().unwrap();
                (start.parse::<u8>().unwrap(), end.parse::<u8>().unwrap())
            })
            .map(|(s, e)| {
                let mut set = HashSet::new();
                for i in s..=e {
                    set.insert(i);
                }
                set
            });
        Self(it.next().unwrap(), it.next().unwrap())
    }

    pub fn has_full_overlap(&self) -> bool {
        self.0.difference(&self.1).count() == 0 || self.1.difference(&self.0).count() == 0
    }

    pub fn has_partial_overlap(&self) -> bool {
        self.0.intersection(&self.1).count() > 0 || self.1.intersection(&self.0).count() > 0
    }
}

fn main() {
    let input = aoc::lines("./input_p1.txt");
    let elf_pairs: Vec<ElfCleanPair> = input.iter().map(ElfCleanPair::from_line).collect();
    let p1 = elf_pairs.iter().filter(|x| x.has_full_overlap()).count();
    println!("p1: {}", p1);
    let p2 = elf_pairs.iter().filter(|x| x.has_partial_overlap()).count();
    println!("p2: {}", p2);
}

#[cfg(test)]
mod test_day_04 {
    use crate::*;
}
