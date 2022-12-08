
fn main() {
    let input = parse_input(include_str!("../input/day8.txt"));

    let result = do_left(&input).iter().zip(do_right(&input)).zip(do_top(&input)).zip(do_bottom(&input))
        .map(|(((a, b), c), d)| a.iter().enumerate()
            .map(|(i, x)| (x + b[i] + c[i] + d[i]).min(1)).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    let tree_count: u32 = result.iter().map(|line| line.iter().sum::<u32>()).sum();

    for line in result {
        for num in line {
            print!("{}", num);
        }
        println!();
    }

    println!("{:?}", tree_count);
}

fn parse_input(s: &str) -> Vec<Vec<u32>>
{
    s.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect()
}

fn do_left(input: &Vec<Vec<u32>>) -> Vec<Vec<u32>>
{
    input.iter()
        .map(|line| {
            let mut max = 0;
            let mut result = Vec::new();
            for (i, num) in line.iter().enumerate() {
                if *num > max || i == 0 {max = *num; result.push(1);}
                else {result.push(0)}
            }
            result
        }
        ).collect()
}

fn do_right(input: &Vec<Vec<u32>>) -> Vec<Vec<u32>>
{
    input.iter()
        .map(|line| {
            let mut max = 0;
            let mut result = Vec::new();
            for (i, num) in line.iter().rev().enumerate() {
                if *num > max || i == 0 {max = *num; result.push(1);}
                else {result.push(0)}
            }
            result.reverse();
            result
        }
        ).collect()
}

fn do_top(input: &Vec<Vec<u32>>) -> Vec<Vec<u32>>
{
    let mut result: Vec<Vec<u32>> = input.iter().map(|line| line.iter().map(|c| 0).collect::<Vec<u32>>()).collect();
    for column in 0..result[0].len() {
        let mut max = 0;
        for row in 0..result.len() {
            if input[row][column] > max {
                max = input[row][column];
                result[row][column] = 1;
            }
        }
    }
    for i in 0..result[0].len() {
        result[0][i] = 1
    }
    result
}

fn do_bottom(input: &Vec<Vec<u32>>) -> Vec<Vec<u32>>
{
    let mut result: Vec<Vec<u32>> = input.iter().map(|line| line.iter().map(|c| 0).collect::<Vec<u32>>()).collect();
    for column in 0..result[0].len() {
        let mut max = 0;
        for row in 0..result.len() {
            let index = result.len() - row - 1;
            if input[index][column] > max {
                max = input[index][column];
                result[index][column] = 1;
            }
        }
    }
    let len = result.len();
    for i in 0..result[0].len() {
        result[len-1][i] = 1
    }
    result
}
