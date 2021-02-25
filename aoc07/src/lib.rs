use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// EXAMPLE INPUT:
// Step C must be finished before step A can begin.
// Step C must be finished before step F can begin.
// Step A must be finished before step B can begin.
// Step A must be finished before step D can begin.
// Step B must be finished before step E can begin.
// Step D must be finished before step E can begin.
// Step F must be finished before step E can begin.
//
//   -->A--->B--
//  /    \      \
//  C      -->D----->E
//  \           /
//   ---->F-----
//
// When reading the input from left to right we can see that it defines a directed acyclic graph.
// C: A, F
// A: B, D
// B: E
// D: E
// F: E
//
// When reading it from right to left we can see that it is easy to construct a dependency list.
// A: C
// F: C
// B: A
// D: A
// E: B, D, F
//
// We can maintain this dependency list and a set of visited nodes while performing a search on the
// graph, we only add a node into the queue if our dependency list allows it, and we visit nodes in
// our queue in alphabetical order.

pub struct Graph {
    start: Vec<u8>,
    adjacencies: HashMap<u8, Vec<u8>>,
    dependencies: HashMap<u8, HashSet<u8>>,
}

impl Graph {
    pub fn new(input: &[Adjacent]) -> Self {
        let mut adjacencies: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut dependencies: HashMap<u8, HashSet<u8>> = HashMap::new();
        let mut all_dependent: HashSet<u8> = HashSet::new();

        for adjacent in input {
            let list = adjacencies.entry(adjacent.0).or_insert_with(|| vec![]);
            list.push(adjacent.1);
            let set = dependencies
                .entry(adjacent.1)
                .or_insert_with(|| HashSet::new());
            set.insert(adjacent.0);
            all_dependent.insert(adjacent.1);
        }

        let start: Vec<u8> = adjacencies
            .keys()
            .filter(|node| !all_dependent.contains(node))
            .copied()
            .collect();

        Graph {
            start,
            adjacencies,
            dependencies,
        }
    }

    pub fn step_order(&self, order: &mut Vec<u8>) {
        let mut visited: HashSet<u8> = HashSet::new();
        let mut queue: Vec<u8> = self.start.iter().copied().collect();
        queue.sort();
        queue.reverse();

        while let Some(current) = queue.pop() {
            visited.insert(current);
            order.push(current);

            if let Some(adjacencies) = self.adjacencies.get(&current) {
                for adjacent in adjacencies {
                    if self.visitable(adjacent, &visited) {
                        queue.push(*adjacent);
                    }
                }
            }

            // sort and have the first item at the top to be popped
            queue.sort();
            queue.dedup();
            queue.reverse();
        }
    }

    // fills order up with the order in which the jobs are completed and returns the total time
    // taken to complete the jobs.
    pub fn with_workers(&self, order: &mut Vec<u8>) -> u32 {
        let mut completed: HashSet<u8> = HashSet::new();
        let mut workers = Workers::new(5);
        let mut time_elapsed = 0;
        let mut queue: Vec<u8> = self.start.iter().copied().collect();
        queue.sort();
        queue.reverse();

        loop {
            if queue.is_empty() && workers.all_idle() {
                break;
            }

            for id in workers.available() {
                match queue.pop() {
                    None => break,
                    Some(step) => {
                        workers.assign_work(id, step);
                    }
                }
            }

            workers.do_work(order, &mut completed);

            for step in completed.iter() {
                if let Some(adjacencies) = self.adjacencies.get(step) {
                    for next in adjacencies.iter() {
                        if self.visitable(next, &completed) && !workers.in_progress(next) {
                            queue.push(*next);
                        }
                    }
                }
            }

            queue.sort();
            queue.dedup();
            queue.reverse();

            time_elapsed += 1;
        }

        time_elapsed
    }

    fn visitable(&self, to_visit: &u8, visited: &HashSet<u8>) -> bool {
        if visited.contains(&to_visit) {
            return false;
        }
        // If a node in the the dependencies of to_visit has not yet been visited then we cannot
        // add to_visit to the queue (it is not visitable yet).
        if let Some(dependencies) = self.dependencies.get(to_visit) {
            return dependencies.iter().all(|dep| visited.contains(dep));
        }
        true
    }
}

pub struct Adjacent(u8, u8);

impl FromStr for Adjacent {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.")
                    .unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from("unrecognized statement")),
            Some(caps) => caps,
        };

        // NOTE TO SELF: use caps[1] because the whole match is stored at index 0
        Ok(Adjacent(caps[1].as_bytes()[0], caps[2].as_bytes()[0]))
    }
}

pub struct Workers {
    workers: Vec<Status>,
}

type ID = usize;

impl Workers {
    pub fn new(count: usize) -> Self {
        let workers = vec![Status::Idle; count];

        Workers { workers }
    }

    pub fn available(&self) -> Vec<ID> {
        self.workers
            .iter()
            .enumerate()
            .filter(|&w| *w.1 == Status::Idle)
            .map(|w| w.0)
            .collect()
    }

    pub fn all_idle(&self) -> bool {
        self.workers.iter().all(|&w| w == Status::Idle)
    }

    pub fn assign_work(&mut self, id: ID, step: u8) {
        let worker = &mut self.workers[id];

        assert!(*worker == Status::Idle, "worker with id {} is busy", id);

        *worker = Status::Working {
            step,
            remaining: step - b'A' + 1 + 60,
        };
    }

    pub fn do_work(&mut self, order: &mut Vec<u8>, completed: &mut HashSet<u8>) {
        for id in 0..self.workers.len() {
            let mut finished = false;
            match self.workers[id] {
                Status::Idle => {}
                Status::Working {
                    step,
                    ref mut remaining,
                } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        finished = true;
                        completed.insert(step);
                        order.push(step);
                    }
                }
            }

            if finished {
                self.workers[id] = Status::Idle;
            }
        }
    }

    pub fn in_progress(&self, step_to_check: &u8) -> bool {
        self.workers.iter().any(|&w| match w {
            Status::Working { step, .. } => step == *step_to_check,
            _ => false,
        })
    }
}

// A worker is defined by its status, i.e. whether it's working or not, and if it is working, then
// where it is working and how much longer it will be working for.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Idle,
    Working { step: u8, remaining: u8 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.";

        let mut adjacencies: Vec<Adjacent> = Vec::new();

        for line in input.lines() {
            adjacencies.push(line.parse().unwrap());
        }

        let graph = Graph::new(&adjacencies);
        let mut order: Vec<u8> = vec![];
        graph.step_order(&mut order);
        assert_eq!(std::str::from_utf8(&order).unwrap(), "CABDFE");
    }
}
