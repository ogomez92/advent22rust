mod file_utils;
mod stack;
use regex::Regex;
use stack::Stack;

fn main() {
    let mut stacks: Vec<Stack> = Vec::new();

    if let Ok(lines) = file_utils::read_lines("input.txt") {
        for current_line in lines {
            if let Ok(line) = current_line {
                if !line.trim().is_empty() {
                    let line_without_spaces = line.replace(" ", "");
                    if line_without_spaces.chars().all(|c| c.is_numeric()) {
                        stacks = get_empty_stacks(line);
                        break;
                    }
                }
            }
        }
    }

    if let Ok(lines) = file_utils::read_lines("input.txt") {
        for current_line in lines {
            if let Ok(line) = current_line {
                if !line.trim().is_empty() {
                    if line.contains("[") {
                        stacks = parse_crates_into_stacks(line, stacks.clone());
                    } else if line.contains("move") {
                        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
                        let captures = re.captures(&line).unwrap();
                        let crate_amount =
                            captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
                        let from_stack = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
                        let to_stack = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();

                        stacks = move_crates(crate_amount, from_stack, to_stack, stacks.clone());
                    }
                }
            }
        }
    }

    let stack_tops = get_top_crates_for_stacks(stacks.clone());
    println!("top crates: {}", stack_tops)
}

fn get_empty_stacks(text: String) -> Vec<Stack> {
    let mut the_stacks: Vec<Stack> = Vec::new();
    let mut counter: i16 = -1;
    let mut last_was_numeric = false;

    for c in text.chars() {
        counter += 1;

        if c.is_numeric() && !last_was_numeric {
            let new_stack = Stack {
                crates: Vec::new(),
                position_in_string: counter as u32,
                stack_number: c.to_digit(10).unwrap(),
            };

            the_stacks.push(new_stack);

            last_was_numeric = true;
        } else {
            last_was_numeric = false;
        }
    }

    return the_stacks;
}

fn parse_crates_into_stacks(line: String, mut stacks: Vec<Stack>) -> Vec<Stack> {
    let re = Regex::new(r"\[").unwrap();
    for bracket in re.find_iter(&line) {
        let stack_index = bracket.start() + 1; // Letter matches stack, not bracket
        let crate_character = line.chars().nth(stack_index).unwrap();
        println!("parse {} at {}", crate_character, stack_index);
        let stack = get_stack_at_position(stack_index as u32, stacks.clone());

        match stack {
            Some(mut stack) => {
                stack.crates.push(crate_character);
                let index_in_vec = stack.stack_number as usize - 1;
                stacks[index_in_vec] = stack;
            }
            _ => panic!("bad stack, not found for position {}", stack_index),
        }
    }

    return stacks;
}

fn get_stack_by_number(position: u32, stacks: Vec<Stack>) -> Option<Stack> {
    for stack in stacks {
        if stack.stack_number == position {
            return Some(stack);
        }
    }
    return None;
}

fn get_stack_at_position(position: u32, stacks: Vec<Stack>) -> Option<Stack> {
    for stack in stacks {
        if stack.position_in_string == position {
            return Some(stack);
        }
    }
    return None;
}

fn move_crates(
    crate_amount: u32,
    from_stack: u32,
    to_stack: u32,
    mut stacks: Vec<Stack>,
) -> Vec<Stack> {
    let from = get_stack_by_number(from_stack, stacks.clone());
    let to = get_stack_by_number(to_stack, stacks.clone());

    let mut from_stack = match from {
        Some(stack) => stack,
        _ => panic!("bad stack, not found for position {}", from_stack),
    };

    let mut to_stack = match to {
        Some(stack) => stack,
        _ => panic!("bad stack, not found for position {}", to_stack),
    };

    let mut crates_to_move = Vec::new();

    for _ in 0..crate_amount {
        // Remove the first element
        let crate_to_move = from_stack.crates.remove(0);
        crates_to_move.push(crate_to_move);
    }

    for crate_to_move in crates_to_move {
        to_stack.crates.insert(0, crate_to_move);
    }

    stacks[from_stack.stack_number as usize - 1] = from_stack.clone();
    stacks[to_stack.stack_number as usize - 1] = to_stack.clone();

    return stacks;
}

fn get_top_crates_for_stacks(stacks: Vec<Stack>) -> String {
    let mut top_crates = String::new();

    for stack in stacks {
        let top_crate = stack.crates.first();

        match top_crate {
            Some(top_crate) => top_crates.push(*top_crate),
            _ => (),
        }
    }

    return top_crates;
}
