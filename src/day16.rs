use std::collections::{HashMap, HashSet};
use regex::Regex;

struct Valve {
    name: String,
    flow_rate: usize,
    connected: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    visited: HashSet<String>,
    current_flow_rate: usize,
    total_flow: usize,
    opening: bool,
    current_valve: String,
}

fn main() {
    let input = std::fs::read_to_string("data/day16.txt").unwrap();

    let pattern = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunne\w+ lea\w+ to valv\w+ (.+)").unwrap();

    let mut valves = HashMap::new();

    for line in input.lines() {
        let captures = pattern.captures(line).unwrap();
        let name = captures[1].to_string();
        let flow_rate = captures[2].parse::<usize>().unwrap();
        let connected = captures[3].split(", ").map(|x| x.to_string()).collect::<Vec<String>>();
        let valve = Valve { name: name.clone(), flow_rate, connected };
        valves.insert(name, valve);
    }

    let mut states = Vec::new();
    states.push(State { visited: HashSet::new(), current_flow_rate: 0, opening: false, current_valve: "AA".to_string(), total_flow: 0 });

    for _ in 0..30 {
        let mut new_states = Vec::new();
        for state in states {
            if state.opening {
                let mut new_state = state.clone();
                new_state.opening = false;
                new_state.total_flow += state.current_flow_rate;
                new_state.current_flow_rate += valves[&state.current_valve].flow_rate;
                new_states.push(new_state);
                continue
            }
            let mut did_something = false;
            for valve in &valves[&state.current_valve].connected {
                let valve = &valves[valve];
                if state.visited.contains(&valve.name) {
                    continue;
                }
                did_something = true;
                let mut new_state = state.clone();

                new_state.visited.insert(valve.name.clone());
                new_state.current_valve = valve.name.clone();
                new_state.total_flow += state.current_flow_rate;
                new_state.opening = valve.flow_rate != 0;
                new_states.push(new_state);
            }
            if !did_something {
                let mut new_state = state.clone();
                new_state.total_flow += state.current_flow_rate;
                new_states.push(new_state);
            }
        }
        states = new_states;
        println!("{}", states.iter().map(|x| x.total_flow).max().unwrap());
    }

    println!("{}", states.iter().map(|x| x.total_flow).max().unwrap());
}