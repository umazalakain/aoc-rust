use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Goal {
    Win,
    Draw,
    Lose
}

const ORDER: [Move; 3] = [Move::Scissors, Move::Rock, Move::Paper];

fn goal_shift(pos: usize, goal: &Goal) -> usize {
    let shift: isize = match goal {
        Goal::Win => 1,
        Goal::Draw => 0,
        Goal::Lose => -1
    };
    ((pos % ORDER.len()) as isize + shift + ORDER.len() as isize) as usize % ORDER.len()
}

fn answer_move<'a>(opponents: &Move, goal: &Goal) -> &'a Move {
    let pos = ORDER.iter().position(|m| m == opponents).unwrap();
    ORDER.iter().skip(goal_shift(pos, goal)).next().unwrap()
}

fn score_move(own: &Move, opponents: &Move) -> i64 {
    let order: Vec<(&Move, &Move)> = ORDER.iter().zip(ORDER.iter().cycle().skip(1)).collect();
    if own == opponents {
        3
    } else if order.contains(&(own, opponents)) {
        0
    } else {
        6
    }
}

fn score_hand(own: &Move) -> i64 {
    match own {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    }
}

fn score(own: &Move, opponents: &Move) -> i64 {
    score_move(own, opponents) + score_hand(own)
}

fn parse_tuple<'a, 'b, A, B, FA, FB>(parse_a: FA, parse_b: FB, line: &str) -> Option<(&'a A, &'b B)>
    where
        FA: Fn(&str) -> Option<&'a A>,
        FB: Fn(&str) -> Option<&'b B> {

    match line.split_whitespace().collect::<Vec<_>>()[..] {
        [a, b] => match (parse_a(a), parse_b(b)) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None
        },
        _ => None
    }
}

fn main() {
    let lookup_opp: HashMap<&str, Move> = [
        ("A", Move::Rock),
        ("B", Move::Paper),
        ("C", Move::Scissors),
    ].iter().cloned().collect();

    let lookup_own: HashMap<&str, Move> = [
        ("X", Move::Rock),
        ("Y", Move::Paper),
        ("Z", Move::Scissors),
    ].iter().cloned().collect();

    let lookup_goal: HashMap<&str, Goal> = [
        ("X", Goal::Lose),
        ("Y", Goal::Draw),
        ("Z", Goal::Win),
    ].iter().cloned().collect();

    let part1 = include_str!("input")
        .lines()
        .flat_map(|line| parse_tuple(|v| lookup_opp.get(v), |v| lookup_own.get(v), line))
        .map(|(opp, own)| score(own, opp))
        .sum::<i64>();

    let part2 = include_str!("input")
        .lines()
        .flat_map(|line| parse_tuple(|v| lookup_opp.get(v), |v| lookup_goal.get(v), line))
        .map(|(opp, goal)| score(answer_move(opp, goal), opp))
        .sum::<i64>();
    
    println!("Part 1: {part1:?}");
    println!("Part 2: {part2:?}");
}
