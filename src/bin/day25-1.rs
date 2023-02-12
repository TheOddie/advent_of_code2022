
fn main() {
    let input = parse_input(include_str!("../input/test.txt"));

    to_snafu(solve(&input));
}

fn solve(snafus: &Vec<Vec<i8>>) -> i128 {
    let powers_of_five = (0..22).map(|x| 5_i128.pow(x)).collect::<Vec<i128>>();

    snafus.iter()
        .map(|num| num.iter()
            .enumerate()
            .map(|(i, &a)| a as i128 * powers_of_five[i])
            .sum::<i128>())
        .sum()
}

fn to_snafu(num: i128) {
    println!("{}", num);
    let mut num = num;
    let powers_of_five = (0..22).map(|x| 5_i128.pow(x)).collect::<Vec<i128>>();
    let mut result = Vec::new();

    for i in (0..22).rev() {
        let r = num / powers_of_five[i];
        result.push(r);
        num -= r * powers_of_five[i];
    }

    while !result.iter().all(|&x| -2 <= x && x <= 2) {
        let last_result = result.clone();
        for (i, &a) in last_result.iter().enumerate() {
            if a >= 3 {
                result[i-1] += 1;
                result[i] -= 5;
            }
        }
    }

    while result[0] == 0 {
        result.remove(0);
    }

    for num in result {
        let ch = match num {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => unreachable!()
        };
        print!("{}", ch);
    }
}

fn parse_input(s: &str) -> Vec<Vec<i8>>
{
    let mut result = Vec::new();
    for line in s.lines() {
        result.push(
            line.chars().rev().map(|ch| {
                match ch {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => unreachable!()
                }
            }).collect::<Vec<i8>>()
        );
    }
    result
}
