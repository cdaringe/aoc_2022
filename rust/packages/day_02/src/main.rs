use aoc;

#[derive(Clone)]
enum RPC {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Win,
    Draw,
    Lose,
}

pub struct Round(RPC, RPC);

impl Round {
    pub fn outcome(&self) -> Outcome {
        match (&self.1, &self.0) {
            (RPC::Rock, RPC::Paper) => Outcome::Lose,
            (RPC::Rock, RPC::Scissors) => Outcome::Win,
            (RPC::Paper, RPC::Scissors) => Outcome::Lose,
            (RPC::Paper, RPC::Rock) => Outcome::Win,
            (RPC::Scissors, RPC::Rock) => Outcome::Lose,
            (RPC::Scissors, RPC::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }
    pub fn score(&self) -> usize {
        let outcome_score = match self.outcome() {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };
        let hand_score = match self.1 {
            RPC::Rock => 1,
            RPC::Paper => 2,
            RPC::Scissors => 3,
        };
        outcome_score + hand_score
    }
}

trait Score {
    fn score(&self) -> usize;
}

impl Score for Vec<Round> {
    fn score(&self) -> usize {
        self.iter().map(|r| r.score()).sum()
    }
}
pub fn parse_rounds(lines: &Vec<String>) -> Vec<Round> {
    lines.iter().fold(vec![], |mut acc, l| {
        let chars = l.chars().take(3).collect::<Vec<char>>();
        let (a, b) = match &chars[..] {
            &[a, _, b] => (a, b),
            _ => panic!("bogus chars"),
        };
        let p1 = match a {
            'A' => RPC::Rock,
            'B' => RPC::Paper,
            'C' => RPC::Scissors,
            c => panic!("unsupported {}", c),
        };
        let p2 = match b {
            'X' => RPC::Rock,
            'Y' => RPC::Paper,
            'Z' => RPC::Scissors,
            c => panic!("unsupported {}", c),
        };
        acc.push(Round(p1, p2));
        acc
    })
}

pub fn parse_strategy(lines: &Vec<String>) -> Vec<Round> {
    lines.iter().fold(vec![], |mut acc, l| {
        let chars = l.chars().take(3).collect::<Vec<char>>();
        let (a, b) = match &chars[..] {
            &[a, _, b] => (a, b),
            _ => panic!("bogus chars"),
        };
        let p1 = match a {
            'A' => RPC::Rock,
            'B' => RPC::Paper,
            'C' => RPC::Scissors,
            c => panic!("unsupported {}", c),
        };
        let outcome = match b {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            c => panic!("unsupported {}", c),
        };
        let p2: RPC = match (&p1, outcome) {
            (m, Outcome::Draw) => m.clone(),
            (RPC::Paper, Outcome::Win) => RPC::Scissors,
            (RPC::Rock, Outcome::Win) => RPC::Paper,
            (RPC::Scissors, Outcome::Win) => RPC::Rock,
            (RPC::Paper, Outcome::Lose) => RPC::Rock,
            (RPC::Rock, Outcome::Lose) => RPC::Scissors,
            (RPC::Scissors, Outcome::Lose) => RPC::Paper,
        };
        acc.push(Round(p1, p2));
        acc
    })
}

fn main() {
    let input = aoc::lines("./input_p1.txt");
    let mut rounds = parse_rounds(&input);
    println!("p1: {}", rounds.score());
    rounds = parse_strategy(&input);
    println!("p2: {}", rounds.score());
}

#[cfg(test)]
mod test_day_02 {
    use crate::*;

    #[test]
    fn test_demo_input() {
        let lines = aoc::lines("./input_p1_demo.txt");
        let parsed = parse_rounds(&lines);
        let result = parsed.score();
        assert_eq!(result, 15)
    }
}
