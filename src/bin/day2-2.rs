
fn main() {
    let result: u32 = include_str!("../input/day2.txt")
        .lines()
        .map(|x| x.split(' ')
            .map(|y| y.bytes().next().unwrap())
            .collect())
        .map(|x: Vec<u8>| (x[0] - 'A' as u8, x[1] - 'X' as u8))
        .map(|(x, y)| (y * 3 + (((x + y + 2) % 3) + 1)) as u32)
        .sum();

    println!("{:?}", result);
}
