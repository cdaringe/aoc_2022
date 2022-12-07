use aoc;
use std::collections::HashSet;

fn solve(input: &str, num_unique: usize) -> usize {
    let chars = &input.chars().collect::<Vec<char>>();
    &chars
        .windows(num_unique)
        .enumerate()
        .find(|(_, x)| {
            let set: HashSet<char> = x.into_iter().map(|c| *c).collect();
            set.len() == num_unique
        })
        .unwrap()
        .0
        + num_unique
}

fn main() {
    let lines = aoc::lines("./input_p1.txt");
    let input = lines.iter().take(1).next().unwrap();
    println!("p1: {}", solve(&input, 4));
    println!("p2: {}", solve(&input, 14));
}

#[cfg(test)]
mod test_day_05 {
    use crate::*;

    #[test]
    fn test_demo_input_p1() {
        let lines = aoc::lines("./input_p1_demo.txt");
        let input = lines.iter().take(1).next().unwrap();
        assert_eq!(solve(&input, 4), 7);
    }

    #[test]
    fn test_demo_input_p2a() {
        let lines = aoc::lines("./input_p2_demo_a.txt");
        let input = lines.iter().take(1).next().unwrap();
        assert_eq!(solve(&input, 14), 19);
    }
}
