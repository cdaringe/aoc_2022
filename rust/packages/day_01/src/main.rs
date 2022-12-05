use aoc;

pub fn parse(lines: &Vec<String>) -> Vec<usize> {
    lines.iter().fold(vec![0], |mut acc, l| {
        if l == "" {
            acc.push(0);
        } else {
            if let Some(last) = acc.last_mut() {
                *last = *last + l.parse::<usize>().unwrap();
            }
        }
        acc
    })
}

pub fn p1(elf_cals: &Vec<usize>) -> usize {
    *elf_cals.iter().max().unwrap()
}

pub fn p2(elf_cals: &mut Vec<usize>) -> usize {
    elf_cals.sort_unstable();
    elf_cals.iter().rev().take(3).sum()
}

fn main() {
    let input = aoc::lines("./input_p1.txt");
    let mut elf_food = parse(&input);
    println!("p1: {}", p1(&elf_food));
    println!("p2: {}", p2(&mut elf_food));
}

#[cfg(test)]
mod test_day_01 {
    use crate::*;

    #[test]
    fn test_demo_input() {
        let lines = aoc::lines("./input_p1_demo.txt");
        let result = p1(&parse(&lines));
        assert_eq!(result, 24000)
    }
}
