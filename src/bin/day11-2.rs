use scanf::sscanf;

fn main() {
    let input = parse_input(include_str!("../input/day11.txt").replace("\r\n", "\n").as_str());

    println!("{:?}", solve(input));
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: String,
    operand: String,
    test_num: i64,
    test_true: usize,
    test_false: usize,
    num_inspections: i64,
}

impl Monkey {
    fn has_items(&self) -> bool {
        return !self.items.is_empty();
    }

    fn next_item(&mut self) -> i64 {
        self.num_inspections += 1;
        self.items.pop().unwrap()
    }

    fn give_item(&mut self, item: i64) {
        self.items.push(item);
    }
}

fn solve(mut monkeys: Vec<Monkey>) -> i64 {

    let mut manageable_mod = 1;
    for monkey in &monkeys {
        manageable_mod *= monkey.test_num;
    }

    for _ in 0..10_000 {

        let len = monkeys.len();
        for this_monkey_index in 0..len
        {
            let mut monkey = monkeys[this_monkey_index].clone();
            while monkey.has_items()
            {
                let mut item = monkey.next_item();
                let operand: Option<i64> = monkey.operand.parse::<i64>().ok();

                match monkey.operation.as_str() {
                    "*" => {
                        item *= match operand {
                            None => {item}
                            Some(x) => {x}
                        };
                    }
                    "+" => {
                        item += match operand {
                            None => {item}
                            Some(x) => {x}
                        };
                    }
                    _ => {unreachable!()}
                }

                item %= manageable_mod;

                let next_monkey = if item % monkey.test_num == 0 {monkey.test_true} else {monkey.test_false};

                monkeys[next_monkey].give_item(item);
            }
            monkeys[this_monkey_index] = monkey.clone();
        }

    }

    let mut counts = monkeys.iter().map(|m| m.num_inspections).collect::<Vec<i64>>();
    counts.sort();
    counts.reverse();

    counts[0] * counts[1]
}

fn parse_input(s: &str) -> Vec<Monkey>
{
    let mut monkeys = Vec::new();

    for input in s.split("\n\n") {
        let mut num = 0;
        let mut starting_items = String::new();
        let mut operation1 = String::new();
        let mut operation2 = String::new();
        let mut test = 0;
        let mut test_true: usize = 0;
        let mut test_false: usize = 0;

        let mut input_lines = input.lines();

        sscanf!(input_lines.next().unwrap(), "Monkey {}:", num).unwrap();
        sscanf!(input_lines.next().unwrap(), "  Starting items: {}", starting_items).unwrap();
        sscanf!(input_lines.next().unwrap(), "  Operation: new = old {} {}", operation1, operation2).unwrap();
        sscanf!(input_lines.next().unwrap(), "  Test: divisible by {}", test).unwrap();
        sscanf!(input_lines.next().unwrap(), "    If true: throw to monkey {}", test_true).unwrap();
        sscanf!(input_lines.next().unwrap(), "    If false: throw to monkey {}", test_false).unwrap();

        let starting_items = starting_items.split(", ").map(|x| x.parse::<i64>().unwrap()).collect();

        let monkey = Monkey {
            items: starting_items,
            operation: operation1,
            operand: operation2,
            test_num: test,
            test_true,
            test_false,
            num_inspections: 0,
        };

        monkeys.push(monkey);
    }

    monkeys
}
