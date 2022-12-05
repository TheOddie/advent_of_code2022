use scanf::sscanf;

fn main() {
    let (mut stacks, input) = parse_input(include_str!("../input/day5.txt"));

    for instruction in input
    {
        for _ in 0..instruction.0
        {
            let item = stacks[instruction.1 - 1].pop().unwrap();
            stacks[instruction.2 - 1].push(item);
        }
    }

    for stack in stacks
    {
        match stack.last() {
            None => {}
            Some(item) => {
                print!("{}", item);
            }
        }
    }
}

fn parse_input(s: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>)
{
    let mut m = s.split("\r\n\r\n");
    let (stacks_str, instructions_str) = (m.next(), m.next());
    let mut stacks = vec![Vec::new(); 9];
    for line in stacks_str.unwrap().lines()
    {
        let chars = line.chars().collect::<Vec<char>>();
        if chars[1] == '1' {break;}
        for i in 0..9
        {
            match chars.get(1 + 4*i) {
                Some(c) => {
                    match c {
                        ' ' => {}
                        _ => {
                            stacks[i].insert(0, chars[1 + 4*i]);
                        }
                    }
                }
                None => {}
            }
        }
    }
    let instructions = instructions_str.unwrap()
        .lines()
        .map(|line| {
            let mut a: usize = 0;
            let mut b: usize = 0;
            let mut c: usize = 0;

            sscanf!(line, "move {} from {} to {}", a, b, c).unwrap();
            (a, b, c)
        })
        .collect::<Vec<(usize, usize, usize)>>();
    return (stacks, instructions);
}
