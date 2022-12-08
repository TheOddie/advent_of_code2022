
fn main() {
    let input = parse_input(include_str!("../input/day8.txt"));

    let result: Vec<Vec<u32>> = input.iter().enumerate()
        .map(|(y, line)| line.iter().enumerate()
            .map(|(x, num)| count_valid_trees(&input, x, y)).collect::<Vec<u32>>()).collect();

    for line in &result {
        for num in line {
            print!("{:?} ", num);
        }
        println!();
    }

    println!("{}", result.iter().map(|line| line.iter().max().unwrap()).max().unwrap())
}

fn parse_input(s: &str) -> Vec<Vec<u32>>
{
    s.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect()
}

fn count_valid_trees(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> u32
{
    let tree_height = trees[y][x];
    let mut counts = (0, 0, 0, 0);
    for i in (0..x).rev() { // left
        let this = trees[y][i];
        counts.0 += 1;
        if this >= tree_height {
            break;
        }
    }
    for i in (x+1)..trees[0].len() { // right
        let this = trees[y][i];
        counts.1 += 1;
        if this >= tree_height {
            break;
        }
    }
    for i in (0..y).rev() { // top
        let this = trees[i][x];
        counts.2 += 1;
        if this >= tree_height {
            break;
        }
    }
    for i in (y+1)..trees.len() { // bottom
        let this = trees[i][x];
        counts.3 += 1;
        if this >= tree_height {
            break;
        }
    }
    counts.0 * counts.1 * counts.2 * counts.3
}
