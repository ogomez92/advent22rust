mod file_utils;

fn main() {
    if let Ok(lines) = file_utils::read_lines("input.txt") {
        for current_line in lines {
            if let Ok(line) = current_line {
                if !line.trim().is_empty() {}
                let four_different_characters_position = find_first_four_character_marker(&line);

                println!(
                    "Four char sequence found! {}",
                    four_different_characters_position
                )
            }
        }
    }
}

fn find_first_four_character_marker(line: &str) -> usize {
    let mut position = 0;
    let mut last_chars: Vec<char> = Vec::new();

    for c in line.chars() {
        position += 1;
        if last_chars.len() == 4 {
            if !last_chars.contains(&c) && vec_has_unique_elements(last_chars.clone()) {
                println!("hi {:?}", last_chars);
                position -= 1;
                return position;
            }
        }

        last_chars.push(c);

        if last_chars.len() > 4 {
            last_chars.remove(0);
        }
    }

    if position == 0 {
        println!(
            "Error, no four character difference was ever found in this string: {}",
            line
        );
        return 0;
    }

    return position;
}

fn vec_has_unique_elements(vec: Vec<char>) -> bool {
    let mut uniques: Vec<char> = Vec::new();

    for c in vec {
        if uniques.contains(&c) {
            return false;
        }
        uniques.push(c);
    }

    return true;
}
