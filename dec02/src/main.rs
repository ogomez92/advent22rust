use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::vec::Vec;

fn main() {
    let mut total_score = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for current_line in lines {
            if let Ok(line) = current_line {
                if !line.trim().is_empty() {
                    total_score += calculate_round_score(line.trim());
                    println!("{}", total_score)
                }
            }
        }
    }
    println!("{}", total_score)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_round_score(line: &str) -> u32 {
    let mut score = 0;
    let score_lose = 0;
    let score_draw = 3;
    let score_win = 6;

    let my_play = line.chars().nth(2).unwrap();
    match my_play {
        'X' => score += 1,
        'Y' => score += 2,
        'Z' => score += 3,
        _ => println!("wrong play"),
    }

    let their_play = line.chars().nth(0).unwrap();

    match their_play {
        'A' => match my_play {
            'X' => score += score_draw,
            'Y' => score += score_win,
            'Z' => score += score_lose,
            _ => println!("wrong play"),
        },
        'B' => match my_play {
            'X' => score += score_lose,
            'Y' => score += score_draw,
            'Z' => score += score_win,
            _ => println!("wrong play"),
        },
        'C' => match my_play {
            'X' => score += score_win,
            'Y' => score += score_lose,
            'Z' => score += score_draw,
            _ => println!("wrong play"),
        },
        _ => println!("wrong play"),
    }
    score
}
