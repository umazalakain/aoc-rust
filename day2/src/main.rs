use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}


const ORDER: [Move; 4] = [Move::Scissors, Move::Rock, Move::Paper, Move::Scissors];

fn score_move(own: &Move, opponents: &Move) -> i64 {
    if own == opponents {
        3
    } else {
        let order: Vec<(&Move, &Move)> = ORDER.iter().zip(ORDER.iter().skip(1)).collect();
        if order.contains(&(own, opponents)) {
            0
        } else {
            6
        } 
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

fn parse_line<'a>(lookup: &'a HashMap<&str, Move>, line: &str) -> Option<(&'a Move, &'a Move)> {
  match line.split_whitespace().flat_map(|v| lookup.get(v)).collect::<Vec<_>>()[..] {
    [a, b] => Some((a, b)),
    _ => None
  }
}

fn main() {
    let lookup: HashMap<&str, Move> = [
        ("X", Move::Rock),
        ("Y", Move::Paper),
        ("Z", Move::Scissors),
        ("A", Move::Rock),
        ("B", Move::Paper),
        ("C", Move::Scissors),
    ].iter().cloned().collect();


    let total = include_str!("input")
        .lines()
        .flat_map(|line| parse_line(&lookup, line))
        .map(|(opp, own)| score(own, opp))
        .sum::<i64>();

    
    println!("{total:?}")
}
