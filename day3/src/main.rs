use itertools::Itertools;
use std::collections::HashSet;
use core::hash::Hash;

fn split_half(input: &str) -> Option<(&str, &str)> {
    match input.len() {
        l if l % 2 == 0 => Some(input.split_at(l / 2)),
        _ => None
    }
}

fn intersection<A>(mut sets: Vec<HashSet<A>>) -> HashSet<A> where A: Eq + Hash {
    match sets.len() {
        0 => HashSet::new(),
        1 => sets.pop().unwrap(),
        _ => {
            let mut result = sets.pop().unwrap();
            result.retain(|item| { sets.iter().all(|set| set.contains(item)) });
            result
        }
    }
}

fn common_item(group: Vec<&str>) -> Option<char> {
    let sets = group
        .into_iter()
        .map(|g| HashSet::from_iter(g.chars()))
        .collect_vec();
    intersection(sets).into_iter().exactly_one().ok()
}

fn score(c: char) -> Option<u64> {
    (b'a'..=b'z').chain(b'A'..=b'Z').position(|p| p == c as u8).map(|v| (v+1) as u64)
}

fn main() {
    let part1: Option<u64> = include_str!("input.txt")
        .lines()
        .map(|line| {
            split_half(line)
                .and_then(|(left, right)| common_item(vec![left, right]))
                .and_then(|v| score(v))
        })
        .collect::<Option<Vec<_>>>()
        .and_then(|xs| xs.iter().sum1());

    let part2: Option<u64> = include_str!("input.txt")
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| 
            common_item(group.collect_vec()).and_then(score)
        )
        .collect::<Option<Vec<_>>>()
        .and_then(|xs| xs.iter().sum1());

    println!("{part1:?}");
    println!("{part2:?}");
}