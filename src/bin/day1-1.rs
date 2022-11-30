
fn main()
{
    let input = include_str!("../input/day1.txt")
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("Hello World! {:?}", input);
}
