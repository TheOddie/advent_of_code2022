
fn main()
{
    let input = include_str!("../input/day1.txt").lines();
    let mut elf_num = 1;
    let mut nums = vec![0];

    for line in input
    {
        let num = match line.parse::<u32>() {
            Ok(v) => {v}
            Err(_) => {elf_num += 1; nums.push(0); 0}
        };
        nums[elf_num - 1] += num;
    }

    println!("{:?}", nums.iter().max().unwrap());
}
