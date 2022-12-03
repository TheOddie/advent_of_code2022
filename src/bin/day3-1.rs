
fn main() {
    let elves = include_str!("../input/day3.txt")
        .lines()
        .map(|x| x.chars().map(|x| x).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let midpoints = elves
        .iter()
        .map(|x| x.len() / 2)
        .collect::<Vec<usize>>();
    let result = elves
        .iter()
        .zip(midpoints)
        .map(|(v, i)| v.split_at(i))
        .map(|(a, b)| similarity(a, b))
        .map(|x| to_priority(x))
        .sum::<u32>();

    println!("{:?}", result);
}

fn similarity(a: &[char], b: &[char]) -> char
{
    /* just brute force it */
    for i in a.iter()
    {
        for j in b.iter()
        {
            if i.eq(j)
            {
                return *i;
            }
        }
    }
    unreachable!()
}

fn to_priority(c: char) -> u32
{
    return if 'a' <= c && c <= 'z'
    {
        1 + (c as u32 - 'a' as u32)
    } else {
        27 + (c as u32 - 'A' as u32)
    }
}
