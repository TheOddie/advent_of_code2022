use scanf::sscanf;

fn main() {
    let input = parse_input(include_str!("../input/day10.txt"));

    solve(input);
}

fn parse_input(s: &str) -> Vec<&str>
{
    s.lines().collect()
}

fn solve(input: Vec<&str>) {
    let mut cycle = 1;
    let mut current_instruction: usize = 0;
    let mut x = 1;

    while current_instruction < input.len() {
        let mut v = 1;

        if sscanf!(input[current_instruction], "addx {}", v).is_ok() {
            update_signal_strength(cycle, x);
            cycle += 1;
            update_signal_strength(cycle, x);
            cycle += 1;
            x += v;
        } else {
            update_signal_strength(cycle, x);
            cycle += 1;
        }

        current_instruction += 1;
    }
}

fn update_signal_strength(cycle: i32, x: i32) {
    print!("{}", if x.abs_diff((cycle - 1) % 40) <= 1 { '#' } else { '.' });
    if cycle % 40 == 0 {
        println!();
    }
}
