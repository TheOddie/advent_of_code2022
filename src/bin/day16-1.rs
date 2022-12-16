use std::collections::HashMap;
use std::time::Instant;
use scanf::sscanf;

fn main()
{
    let start = Instant::now();
    let valves = parse_input(include_str!("../input/day16.txt"));

    let mut index_map = HashMap::new();
    valves.iter()
        .enumerate()
        .for_each(|(i, (v, _))| {
            index_map.insert(v.clone(), i);
        });

    let indexed_valves = valves.iter()
        .map(|(_, (f, v))| (*f, v.iter()
            .map(|x| *index_map.get(x).unwrap()).collect::<Vec<usize>>()))
        .collect::<Vec<(i32, Vec<usize>)>>();

    let floyd_w = floyd_warshall(&indexed_valves);

    let flow_rates = indexed_valves.iter()
        .map(|v| v.0)
        .collect::<Vec<i32>>();

    // the useful valves are all those that don't have a flow rate of zero,
    // we are looking for the path through a subset of these values that
    // maximises the pressure released
    let useful_valves = indexed_valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.0 > 0)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let start_position = *index_map.get(&"AA".to_string()).unwrap();

    println!("{:?}", dfs(&floyd_w, &flow_rates,
                         start_position,
                         useful_valves,
                         30));

    println!("Done in {}µs", start.elapsed().as_micros());
}

fn dfs(distances: &Vec<Vec<i32>>, flow_rates: &Vec<i32>, current_valve: usize, remaining_valves: Vec<usize>, time_remaining: i32) -> i32
{
    IsolateEach::new(remaining_valves)
        .filter(|(val, _)| distances[current_valve][*val] < time_remaining)
        .map(|(val, rest)| flow_rates[val] * (time_remaining - distances[current_valve][val] - 1) +
            dfs(distances, flow_rates, val, rest, time_remaining - distances[current_valve][val] - 1))
        .max()
        .unwrap_or(0)
}

// finds amount of time it would take to go from any valve to any other valve
fn floyd_warshall(valves: &Vec<(i32, Vec<usize>)>) -> Vec<Vec<i32>>
{
    let mut dist = vec![vec![99; valves.len()]; valves.len()];

    valves.iter()
        .enumerate()
        .for_each(|(i, (_, v))| {
            v.iter().for_each(|j| {
                dist[i][*j] = 1;
            });
        });

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                dist[i][j] = (dist[i][k] + dist[k][j]).min(dist[i][j]);
            }
        }
    }

    dist
}

struct IsolateEach<T: Clone> {
    i: usize,
    v: Vec<T>
}

impl<T: Clone> IsolateEach<T> {
    fn new(v: Vec<T>) -> Self {
        IsolateEach {
            i: 0,
            v
        }
    }
}

impl<T: Clone> Iterator for IsolateEach<T>
{
    type Item = (T, Vec<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.v.len() {return None;}
        let mut r = self.v.clone();
        r.remove(self.i);
        let result = Some((self.v[self.i].clone(), r));
        self.i += 1;
        result
    }
}

fn parse_input(s: &str) -> Vec<(String, (i32, Vec<String>))>
{
    let mut valves = Vec::new();

    for line in s.lines() {
        let mut name= String::new();
        let mut flow_rate = 0;
        let mut _t = String::new();
        let mut _v = String::new();
        let mut tunnels_str = String::new();

        sscanf!(line, "Valve {} has flow rate={}; {} to {} {}", name, flow_rate, _t, _v, tunnels_str).unwrap();

        let tunnels = tunnels_str.split(", ").map(|x| x.to_string()).collect::<Vec<String>>();
        valves.push((name, (flow_rate, tunnels)));
    }

    valves
}
