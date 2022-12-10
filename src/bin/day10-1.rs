use scanf::sscanf;

fn main() {
    let input = parse_input(include_str!("../input/day10.txt"));

    println!("{:?}", solve(input));
}

fn parse_input(s: &str) -> Vec<&str>
{
    s.lines().collect()
}

fn solve(input: Vec<&str>) -> i32 {
    let mut cycle = 1;
    let mut current_instruction: usize = 0;
    let mut x = 1;
    let mut signal_strength = 0;

    while current_instruction < input.len() {
        let mut v = 1;

        if sscanf!(input[current_instruction], "addx {}", v).is_ok() {
            cycle += 1;
            signal_strength += update_signal_strength(cycle, x);
            x += v;
        }

        current_instruction += 1;
        cycle += 1;
        signal_strength += update_signal_strength(cycle, x);
    }

    signal_strength
}

fn update_signal_strength(cycle: i32, x: i32) -> i32 {
    if cycle % 40 != 20 {
        return 0;
    }
    return cycle * x;
}
