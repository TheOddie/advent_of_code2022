
fn main() {
    let input = parse_input(include_str!("../input/day4.txt"))
        .iter()
        .map(|x| if ((x[0][0] <= x[1][0]) && (x[0][1] >= x[1][1])) || ((x[0][0] >= x[1][0]) && (x[0][1] <= x[1][1])) {1} else {0})
        .sum::<u32>();

    println!("{:?}", input);
}

fn parse_input(s: &str) -> Vec<Vec<Vec<u8>>>
{
    let result = s.lines()
        .map(|x| x.split(","))
        .map(|x| x.map(|y| y.split("-"))
            .map(|y| y.map(|z| z.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>())
        .collect::<Vec<Vec<Vec<u8>>>>();

    return result;
}
