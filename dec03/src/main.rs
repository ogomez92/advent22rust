mod file_utils;

fn main() {
    let mut sum_of_priorities = 0;
    if let Ok(lines) = file_utils::read_lines("input.txt") {
        for current_line in lines {
            if let Ok(line) = current_line {
                if !line.trim().is_empty() {
                    sum_of_priorities += get_priority_for_line(&line.trim());
                }
            }
        }
    }

    println!("total sum of priorities: {}", sum_of_priorities)
}

fn get_priority_for_line(line: &str) -> u32 {
    let mut priority_for_this_line = 0;
    let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
    // find chars contained in first and second
    let mut common_chars = String::new();
    for c in first_compartment.chars() {
        if second_compartment.contains(c) {
            common_chars.push(c);
            let priority = get_priority_for_char(c);
            println!("priority for {}: {}", c, priority);
            priority_for_this_line += priority;
            break
        }
    }
    priority_for_this_line
}

fn get_priority_for_char(c: char) -> u32 {
let priorities = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
if priorities.contains(c) {
    let priority_index = priorities.find(c).expect("Value not found in priority table");
    priority_index as u32
} else {
    panic!("{} is not a valid char", c);
}
}