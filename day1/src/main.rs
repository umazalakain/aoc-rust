use itertools::Itertools;
use std::cmp::Reverse;

fn main() {
    let total = include_str!("input.txt")
        .lines()
        .map(|v: &str| v.parse::<u64>())
        .group_by(|v| v.is_ok())
        .into_iter()
        .filter_map(|g| g.1.into_iter().flatten().sum1::<u64>())
        // https://fasterthanli.me/series/advent-of-code-2022/part-1#improving-the-batching-solution
        // .batching(|it| it.map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|r| r.0)
        .sum::<u64>();

    println!("{total:?}");
}