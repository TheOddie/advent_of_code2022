use std::collections::HashSet;

fn main() {
    let input = parse_input(include_str!("../input/day6.txt"));

    let len_sequence = 14;

    let mut index = len_sequence;
    let mut s = take_slice(&input, index, len_sequence);
    while !is_marker(s.as_slice()) {
        index += 1;
        s = take_slice(&input, index, len_sequence);
    }

    println!("{:?}", index);
}

fn parse_input(s: &str) -> Vec<char>
{
    return s.chars().collect::<Vec<char>>();
}

fn is_marker(s: &[char]) -> bool
{
    let mut result = HashSet::new();
    for i in s {
        result.insert(i.clone());
    }
    return result.len() == s.len();
}

fn take_slice(input: &Vec<char>, end: usize, length: usize) -> Vec<char>
{
    return input.iter()
        .take(end)
        .rev()
        .take(length)
        .rev()
        .map(|x| x.clone())
        .collect::<Vec<char>>();
}
