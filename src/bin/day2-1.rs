
fn main() {
    let result: u32 = include_str!("../input/day2.txt")
        .lines()
        .map(|x| x.split(' ')
            .map(|y| y.bytes().next().unwrap())
            .collect())
        .map(|x: Vec<u8>| (x[0] - 'A' as u8, x[1] - 'X' as u8))
        .map(|(x, y)| ((y + 1) + (((2 * x + y + 1) % 3) * 3)) as u32)
        .sum();

    println!("{:?}", result);
}
