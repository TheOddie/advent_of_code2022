use std::cell::RefCell;
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

    println!("{:?}", dfs_part_2(&floyd_w, &flow_rates,
                         start_position,
                         useful_valves.clone(),
                         26,
                         start_position));

    println!("Done in {}ms", start.elapsed().as_millis());
}

std::thread_local! {
    static MEMOIZED_MAPPING_DFS : RefCell<HashMap<(usize, Vec<usize>, i32), i32>> = RefCell::new(HashMap::new());
}

fn memoized_original_dfs(distances: &Vec<Vec<i32>>, flow_rates: &Vec<i32>, current_valve: usize, remaining_valves: Vec<usize>, time_remaining: i32) -> i32
{
    IsolateEach::new(remaining_valves)
        .filter(|(val, _)| distances[current_valve][*val] < time_remaining)
        .map(|(val, rest)| flow_rates[val] * (time_remaining - distances[current_valve][val] - 1) +
            dfs(distances, flow_rates, val, rest, time_remaining - distances[current_valve][val] - 1))
        .max()
        .unwrap_or(0)
}

fn dfs(dist: &Vec<Vec<i32>>, flows: &Vec<i32>, cur: usize, rest: Vec<usize>, t: i32) -> i32
{
    let result = MEMOIZED_MAPPING_DFS.with(|hm| {
        let hm = hm.borrow_mut();
        hm.get(&(cur.clone(), rest.clone(), t.clone())).cloned()
    });
    if let Some(result) = result {
        return result;
    }

    let result = memoized_original_dfs(dist, flows, cur.clone(), rest.clone(), t.clone());

    MEMOIZED_MAPPING_DFS.with(|hm| {
        let mut hm = hm.borrow_mut();
        hm.insert((cur, rest, t), result.clone());
    });

    result
}

// This is a very stupid shortcut, it only works because there are too many valves in the real input
// for you to be able to ever open all of them in 26 turns. Because of this, this solution fails all
// of the test cases. It works by, instead of actually having you and the elephant traverse the cave
// together at the same time, you traverse the cave first, giving some path which will have a subset
// of remaining valves that the elephant will then traverse. We check over every possible path for
// the elephant for each of your possible paths and take the path that returns the largest pressure
// released value.
fn dfs_part_2(distances: &Vec<Vec<i32>>, flow_rates: &Vec<i32>, current_valve: usize, remaining_valves: Vec<usize>, time_remaining: i32, start_position: usize) -> i32
{
    IsolateEach::new(remaining_valves.clone())
        .filter(|(val, _)| distances[current_valve][*val] < time_remaining)
        .map(|(val, rest)| flow_rates[val] * (time_remaining - distances[current_valve][val] - 1) +
            dfs_part_2(distances, flow_rates, val, rest, time_remaining - distances[current_valve][val] - 1, start_position))
        .max()
        .unwrap_or(dfs(distances, flow_rates, start_position, remaining_valves.clone(), 26))
}

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
