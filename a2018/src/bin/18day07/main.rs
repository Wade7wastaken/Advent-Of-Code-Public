use std::{
    array,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use lib::{CollectHashmap, CollectString, indexmap::IndexSet, itertools::Itertools, select};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn get_requirements(input: &str) -> HashMap<u8, HashSet<u8>> {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split_ascii_whitespace();
            let (requirement, step) = select!(iter; 1, 7);

            (step.as_bytes()[0], requirement.as_bytes()[0])
        })
        .collect_hashmap(|v| HashSet::from([v]), HashSet::insert)
}

fn get_initial_todo(requirements: &HashMap<u8, HashSet<u8>>) -> BinaryHeap<Reverse<u8>> {
    requirements
        .values()
        .flatten()
        .unique()
        .filter(|k| !requirements.contains_key(*k))
        .map(|x| Reverse(*x))
        .collect::<BinaryHeap<_>>()
}

fn is_superset(steps_done: &IndexSet<u8>, req: &HashSet<u8>) -> bool {
    req.iter().all(|value| steps_done.contains(value))
}

fn push_new_steps(
    requirements: &mut HashMap<u8, HashSet<u8>>,
    steps_done: &IndexSet<u8>,
    todo: &mut BinaryHeap<Reverse<u8>>,
) {
    let next = requirements
        .iter()
        .filter(|(_, v)| is_superset(steps_done, v))
        .map(|(k, _)| *k)
        .collect_vec();
    for k in next {
        todo.push(Reverse(k));
        requirements.remove(&k);
    }
}

fn part1(input: &str) -> String {
    let mut requirements = get_requirements(input);

    let mut steps_done = IndexSet::new();
    let mut todo = get_initial_todo(&requirements);

    while let Some(step) = todo.pop() {
        steps_done.insert(step.0);
        push_new_steps(&mut requirements, &steps_done, &mut todo);
    }

    steps_done.into_iter().collect_string()
}

#[derive(Debug, Default)]
struct Worker {
    working: bool,
    timer: u8,
    working_on: u8,
}

impl Worker {
    fn work(&mut self) -> Option<u8> {
        if self.timer == 0 {
            self.working = false;
            return Some(self.working_on);
        }
        self.timer -= 1;
        None
    }

    fn take(&mut self, job: u8) {
        self.working = true;
        self.timer = 60 + job - b'A';
        self.working_on = job;
    }
}

fn part2(input: &str) -> u32 {
    let mut workers: [_; 5] = array::from_fn(|_| Worker::default());

    let mut requirements = get_requirements(input);
    let mut steps_done = IndexSet::new();
    let mut todo = get_initial_todo(&requirements);

    let mut time = 0;

    while !requirements.is_empty() || workers.iter().any(|w| w.working) {
        for w in workers.iter_mut().filter(|w| w.working) {
            if let Some(finished) = w.work() {
                steps_done.insert(finished);
            }
        }
        push_new_steps(&mut requirements, &steps_done, &mut todo);
        for w in workers.iter_mut().filter(|w| !w.working) {
            if let Some(next_job) = todo.pop() {
                w.take(next_job.0);
            }
        }
        time += 1;
    }

    time - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), "EFHLMTKQBWAPGIVXSZJRDUYONC");
        assert_eq!(part2(input), 1056);
    }
}
