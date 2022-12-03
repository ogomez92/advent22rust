mod file_utils;

fn main() {
    let mut grouped_lines: Vec<String> = Vec::new();
    let mut sum_of_priorities = 0;
    if let Ok(lines) = file_utils::read_lines("input.txt") {
        let mut line_counter = 0;
        for current_line in lines {
            if let Ok(line) = current_line {
                if !line.trim().is_empty() {
                    line_counter += 1;
                    if line_counter % 3 == 0 {
                        grouped_lines.push(line);
                        let badge = get_badge_for_group(grouped_lines);
                        let priority = get_priority_for_char(badge);
                        sum_of_priorities += priority;
                        grouped_lines = Vec::new();
                    } else {
                        grouped_lines.push(line)
                    }
                }
            }
        }

        println!("sum {}", sum_of_priorities)
    }

    println!("total sum of priorities: {}", sum_of_priorities)
}

fn get_priority_for_char(c: char) -> u32 {
    let priorities = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    if priorities.contains(c) {
        let priority_index = priorities
            .find(c)
            .expect("Value not found in priority table");
        priority_index as u32
    } else {
        panic!("{} is not a valid char", c);
    }
}

fn get_badge_for_group(grouped_lines: Vec<String>) -> char {
    for c in grouped_lines[0].chars() {
        if grouped_lines[1].contains(c) && grouped_lines[2].contains(c) {
            return c
        }
    }

    panic!("no badge found")
}
