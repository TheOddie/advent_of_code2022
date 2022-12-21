use std::collections::HashMap;
use std::time::Instant;
use scanf::sscanf;

fn main() {
    let start = Instant::now();
    let input = parse_input(include_str!("../input/day21.txt"));

    println!("{:?}", evaluate("root", &input));
    println!("Done in {}ms", start.elapsed().as_millis());
}

fn evaluate(key: &str, map: &HashMap<String, MonkeyValue>) -> i64 {
    match map.get(key).unwrap() {
        MonkeyValue::Number(n) => {*n}
        MonkeyValue::Operation(l, o, r) => {
            match o {
                &'+' => evaluate(l, map) + evaluate(r, map),
                &'-' => evaluate(l, map) - evaluate(r, map),
                &'*' => evaluate(l, map) * evaluate(r, map),
                &'/' => evaluate(l, map) / evaluate(r, map),
                _ => unreachable!()
            }
        }
    }
}

fn parse_input(s: &str) -> HashMap<String, MonkeyValue>
{
    let mut monkeys: HashMap<String, MonkeyValue> = HashMap::new();

    for line in s.lines() {
        let mut monkey = String::new();
        let mut value = String::new();

        sscanf!(line, "{}: {}", monkey, value).unwrap();

        if let Ok(number) = value.parse() {
            monkeys.insert(monkey, MonkeyValue::Number(number));
        } else {
            let mut left = String::new();
            let mut right = String::new();
            let mut operation = ' ';

            sscanf!(value.as_str(), "{} {} {}", left, operation, right).unwrap();

            monkeys.insert(monkey, MonkeyValue::Operation(left, operation, right));
        }
    }

    monkeys
}

enum MonkeyValue {
    Number(i64),
    Operation(String, char, String),
}
