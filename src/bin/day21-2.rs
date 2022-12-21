use std::collections::HashMap;
use std::time::Instant;
use scanf::sscanf;

fn main() {
    let start = Instant::now();
    let input = parse_input(include_str!("../input/day21.txt"));

    println!("{:?}", solve(input));
    println!("Done in {}ms", start.elapsed().as_millis());
}

fn solve(monkeys: HashMap<String, MonkeyValue>) -> i64 {
    let mut humn = ("humn".to_string(), 0);

    let mut x1: f64 = 0.0;
    let mut x2: f64 = 1E14; // sufficiently big number that doesn't error

    for _ in 0..10 {
        let y1 = f(x1 as i64, &monkeys);
        let y2 = f(x2 as i64, &monkeys);

        let fy1 = y1.unwrap() as f64;
        let fy2 = y2.unwrap() as f64;

        // draw a line between two arbitrary points on f(x), find where x = 0

        // 0 - y1 / ((y2 - y1)/(x2 - x1)) + x1 = x

        let x = -fy1 * ((x2 - x1) / (fy2 - fy1)) + x1;
        println!("{x} -> {:?}", f(x as i64, &monkeys));
        let mid = (x1 + x2) / 2.0;
        if x > mid {
            x1 = mid;
        } else {
            x2 = mid;
        }
        humn.1 = x as i64 - 5;
    }

    for i in humn.1..humn.1+10 {
        let y = f(i, &monkeys).unwrap();
        println!("{i} -> {:?}", y);
        if y == 0 {
            return i;
        }
    }

    0
}

fn f(x: i64, map: &HashMap<String, MonkeyValue>) -> Option<i64> {
    let mut map = map.clone();
    map.insert("humn".to_string(), MonkeyValue::Number(x));
    evaluate("root", &map)
}

fn evaluate(key: &str, map: &HashMap<String, MonkeyValue>) -> Option<i64> {
    match map.get(key).unwrap() {
        MonkeyValue::Number(n) => {Some(*n)}
        MonkeyValue::Operation(l, o, r) => {
            let left = evaluate(l, map);
            let right = evaluate(r, map);
            if left.is_none() || right.is_none() {
                return None;
            }
            match o {
                &'+' => left.unwrap().checked_add(right.unwrap()),
                &'-' => left.unwrap().checked_sub(right.unwrap()),
                &'*' => left.unwrap().checked_mul(right.unwrap()),
                &'/' => left.unwrap().checked_div(right.unwrap()),
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

            if monkey == "root".to_string() {
                operation = '-';
            }

            monkeys.insert(monkey, MonkeyValue::Operation(left, operation, right));
        }
    }

    monkeys
}

#[derive(Clone, Debug)]
enum MonkeyValue {
    Number(i64),
    Operation(String, char, String),
}
