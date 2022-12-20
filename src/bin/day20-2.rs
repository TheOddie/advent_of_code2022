use std::time::Instant;

fn main() {
    let start = Instant::now();
    let nums = include_str!("../input/day20.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .map(|n: i64| n * 811589153)
        .collect::<Vec<_>>();

    // let result = apply_n(mix, &nums, 10);
    let result = mix(&nums, 10);
    println!("{:?}", grove_coord(&result));
    println!("Done in {}ms", start.elapsed().as_millis());
}

fn mix(nums: &Vec<i64>, num_iterations: u32) -> Vec<i64> {
    let mut result = (0..nums.len()).collect::<Vec<_>>();

    for _ in 0..num_iterations {
        for (i, &x) in nums.iter().enumerate() {
            let index = result.iter().position(|&y| y == i).unwrap();
            let new_index = modulo(index as i64 + x, result.len() as i64 - 1) as usize;
            result.remove(index);
            result.insert(new_index, i);
        }
    }

    result.iter().map(|&i| nums[i]).collect()
}

fn grove_coord(nums: &Vec<i64>) -> i64 {
    let index_of_zero = nums.iter().position(|&i| i == 0).unwrap();
    (1..=3).map(|i| nums[(index_of_zero + i * 1000) % nums.len()]).sum()
}

fn apply_n<T: Clone>(func: fn(&T) -> T, data: &T, times: u32) -> T {
    if times == 0 { return data.clone(); }
    func(&apply_n(func, data, times - 1))
}

fn modulo(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}
