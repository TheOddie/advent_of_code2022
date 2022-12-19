use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use scanf::sscanf;

fn main() {
    let start = Instant::now();
    let input = parse_input(include_str!("../input/day19.txt"));
    let time = 24;

    let result: i32 = input.iter()
        .map(|bp| bp.blueprint_num * solve_blueprint(bp, time) as i32)
        .sum();
    println!("{:?}", result);
    println!("Done in {}ms", start.elapsed().as_millis());
}

fn solve_blueprint(blueprint: &Blueprint, time: i16) -> i16
{
    let mut q = VecDeque::new();
    q.push_front(State::initial(time));
    let mut best = 0;
    let mut seen = HashSet::new();

    let max_costs = [blueprint.costs.iter()
        .map(|c| c[0])
        .max()
        .unwrap(),
        blueprint.costs[2][1],
        blueprint.costs[3][2]];

    while q.len() != 0 {
        let mut state = q.pop_front().unwrap();

        best = best.max(state.storage[3]);

        if state.time == 0 { continue; }

        // I can only create one new robot at a time so producing more than I can use is pointless.
        // and will give the same result as not doing it. this means they will have the same state
        // when we check that we have seen them before with the seen set.
        for i in 0..3 {
            state.robots[i] = state.robots[i].min(max_costs[i]);
            state.storage[i] = state.storage[i].min(state.time * max_costs[i] - state.robots[i] * (state.time - 1));
        }

        // It is possible to reach the same state by many different paths,
        // we only want to traverse these paths once each.
        if !seen.insert(state) {
            continue;
        }

        let mut new_state = state.clone();
        new_state.time -= 1;
        for i in 0..4 {
            new_state.storage[i] += new_state.robots[i];
        }

        q.push_back(new_state);
        for i in 0..4 {
            // if we can afford a new robot, make it.
            if state.storage.iter().zip(blueprint.costs[i]).all(|(s, bp)| *s >= bp) {
                let mut new_new_state = new_state.clone();
                for j in 0..4 {
                    new_new_state.storage[j] -= blueprint.costs[i][j];
                }
                new_new_state.robots[i] += 1;
                q.push_back(new_new_state);
            }
        }
    }

    best
}

fn parse_input(s: &str) -> Vec<Blueprint>
{
    let mut blueprints = Vec::new();

    for line in s.lines() {
        let mut blueprint_num: i32 = 0;
        let mut ore_robot_cost: i16 = 0;
        let mut clay_robot_cost: i16 = 0;
        let mut obs_robot_ore_cost: i16 = 0;
        let mut obs_robot_clay_cost: i16 = 0;
        let mut geo_robot_ore_cost: i16 = 0;
        let mut geo_robot_obs_cost: i16 = 0;

        sscanf!(line, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            blueprint_num, ore_robot_cost, clay_robot_cost, obs_robot_ore_cost, obs_robot_clay_cost,
            geo_robot_ore_cost, geo_robot_obs_cost).unwrap();
        blueprints.push(Blueprint {
            blueprint_num,
            //      <ore costs>         <clay costs>         <obsidian costs>    <geode costs>
            costs: [[ore_robot_cost,     0,                   0,                  0], // ore robot
                    [clay_robot_cost,    0,                   0,                  0], // clay robot
                    [obs_robot_ore_cost, obs_robot_clay_cost, 0,                  0], // obsidian robot
                    [geo_robot_ore_cost, 0,                   geo_robot_obs_cost, 0]] // geode robot
        });
    }
    blueprints
}

struct Blueprint {
    blueprint_num: i32,
    costs: [[i16; 4]; 4],
}

// the smaller the numbers we store are, the faster our hashset will be
#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct State {
    storage: [i16; 4],
    robots: [i16; 4],
    time: i16,
}

impl State {
    fn initial(time: i16) -> Self
    {
        State {
            storage: [0, 0, 0, 0],
            robots: [1, 0, 0, 0],
            time,
        }
    }
}
