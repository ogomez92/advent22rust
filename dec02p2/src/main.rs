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

    let desired_outcome = line.chars().nth(2).unwrap();
    let their_play = line.chars().nth(0).unwrap();
    let mut my_play = "";

    match desired_outcome {
        'X' => score += score_lose,
        'Y' => score += score_draw,
        'Z' => score += score_win,
        _ => println!("wrong outcome")
    }
    match their_play {
        'A' => match desired_outcome {
            'X' => my_play="Z",
            'Y' => my_play="X",
            'Z' => my_play="Y",
            _ => println!("wrong play for A {}", desired_outcome),
        },

        'B' => match desired_outcome {
            'X' => my_play="X",
            'Y' => my_play="Y",
            'Z' => my_play="Z",
            _ => println!("wrong play for B {}", desired_outcome),
        },

        'C' => match desired_outcome {
            'X' => my_play="Y",
            'Y' => my_play="Z",
            'Z' => my_play="X",
            _ => println!("wrong play for C {}", desired_outcome),
        },
        _ => println!("wrong enemy play {}", their_play),
    }

    match my_play {
        "X" => score += 1,
        "Y" => score += 2,
        "Z" => score += 3,
        _ => println!("wrong play"),
    }
    score
}
