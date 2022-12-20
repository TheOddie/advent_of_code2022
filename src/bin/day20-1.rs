use std::time::Instant;

fn main() {
    let start = Instant::now();
    let nums = include_str!("../input/day20.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

    println!("{}", grove_coord(mix(&nums)));
    println!("Done in {}ms", start.elapsed().as_millis());
}

fn mix(nums: &Vec<i64>) -> Vec<i64> {
    let mut result = (0..nums.len()).collect::<Vec<_>>();

    for (i, &x) in nums.iter().enumerate() {
        let index = result.iter().position(|&y| y == i).unwrap();
        let new_index = modulo(index as i64 + x, result.len() as i64 - 1) as usize;
        result.remove(index);
        result.insert(new_index, i);
    }

    result.iter().map(|&i| nums[i]).collect()
}

fn grove_coord(nums: Vec<i64>) -> i64 {
    let index_of_zero = nums.iter().position(|&i| i == 0).unwrap();
    (1..=3).map(|i| nums[(index_of_zero + i * 1000) % nums.len()]).sum()
}

fn modulo(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}
