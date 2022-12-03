use std::collections::HashSet;

fn main() {
    let elves = include_str!("../input/day3.txt")
        .lines()
        .map(|x| x.chars().map(|x| x).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut groups = Vec::new();
    for i in 0..(elves.len() / 3)
    {
        let i = 3 * i;
        groups.push((elves[i].clone(), elves[i + 1].clone(), elves[i + 2].clone()))
    }
    let result = groups
        .iter()
        .map(|x| similarity(&x.0, &x.1, &x.2))
        .map(|x| to_priority(x))
        .sum::<u32>();

    println!("{:?}", result);
}

fn similarity(a: &Vec<char>, b: &Vec<char>, c: &Vec<char>) -> char
{
    /* just brute force it */
    for i in a.iter()
    {
        for j in b.iter()
        {
            for k in c.iter() {
                if i.eq(j) && i.eq(k)
                {
                    return *i;
                }
            }
        }
    }
    unreachable!();
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
