use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    // Create vector calories
    let mut calories: Vec<u32> = Vec::new();
    // use read_lines to read input.txt
    if let Ok(lines) = read_lines("input.txt") {
        let mut calory_counter: u32 = 0;

        for current_line in lines {
            if let Ok(line) = current_line {
                if !line.trim().is_empty() {
                    let line = line.parse::<u32>().unwrap();
                    calory_counter += line;
                } else {
                    calories.push(calory_counter);
                    calory_counter = 0
                }
            }
        }
    }

    calories.sort_by(|a, b| b.cmp(a));

    // print the top 3 results
    let mut sum_of_values = 0;

    for i in 0..3 {
        sum_of_values += calories[i]
    }
println!("{}", sum_of_values)
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
