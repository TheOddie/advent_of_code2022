
fn main() {
    let input: u32 = include_str!("../input/day2.txt")
        .lines()
        .map(|x| score_calc(parse_instruction(x)))
        .sum();

    println!("{:?}", input);
}

enum RPS {
    Rock, Paper, Scissors
}

impl RPS {
    fn get_score(self) -> u32 {
        match self {
            RPS::Rock => {1}
            RPS::Paper => {2}
            RPS::Scissors => {3}
        }
    }
}

enum WDL {
    Win, Lose, Draw
}

impl WDL {
    fn get_score(self) -> u32 {
        match self {
            WDL::Win => {6}
            WDL::Lose => {0}
            WDL::Draw => {3}
        }
    }
}

fn parse_instruction(s: &str) -> (RPS, WDL)
{
    let mut chars = s.bytes();
    let opponent = match chars.next() {
        Some(b'A') => RPS::Rock,
        Some(b'B') => RPS::Paper,
        Some(b'C') => RPS::Scissors,
        _ => unreachable!()
    };

    chars.next();

    let you = match chars.next() {
        Some(b'X') => WDL::Lose,
        Some(b'Y') => WDL::Draw,
        Some(b'Z') => WDL::Win,
        _ => unreachable!()
    };

    (opponent, you)
}

fn score_calc(round: (RPS, WDL)) -> u32 {
    let result = match round.0 {
        RPS::Rock => {
            match round.1 {
                WDL::Draw => {RPS::Rock}
                WDL::Win => {RPS::Paper}
                WDL::Lose=> {RPS::Scissors }
            }
        }
        RPS::Paper => {
            match round.1 {
                WDL::Lose => {RPS::Rock}
                WDL::Draw => {RPS::Paper}
                WDL::Win => {RPS::Scissors }
            }
        }
        RPS::Scissors => {
            match round.1 {
                WDL::Win => {RPS::Rock}
                WDL::Lose => {RPS::Paper}
                WDL::Draw => {RPS::Scissors }
            }
        }
    };

    return result.get_score() + round.1.get_score();
}
