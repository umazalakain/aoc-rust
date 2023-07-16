use itertools::Itertools;

fn split_half(input: &str) -> Option<(&str, &str)> {
    match input.len() {
        l if l % 2 == 0 => Some(input.split_at(l / 2)),
        _ => None
    }
}

fn common_item(left: &str, right: &str) -> Option<char> {
    left.chars().unique().filter(|v| right.chars().contains(v)).exactly_one().ok()
}


fn score(c: char) -> Option<u64> {
    (b'a'..=b'z').chain(b'A'..=b'Z').position(|p| p == c as u8).map(|v| (v+1) as u64)
}

fn main() {
    let score: Option<u64> = include_str!("input.txt")
        .lines()
        .map(|line| {
            split_half(line)
                .and_then(|(left, right)| common_item(left, right))
                .and_then(|v| score(v))
        })
        .collect::<Option<Vec<_>>>()
        .and_then(|xs| xs.iter().sum1());

    println!("{score:?}");
}