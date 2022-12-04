use std::collections::HashSet;

mod file_utils;

fn main() {
    let mut double_cleaning_elves = 0;
    if let Ok(lines) = file_utils::read_lines("input.txt") {
        for current_line in lines {
            if let Ok(line) = current_line {
                if !line.trim().is_empty() {
                    let (first_elf, second_elf) = get_assigned_sections_for_pair(line.trim());
                    println!("{}", line);
                    if elves_fully_contained(first_elf, second_elf) {
                        double_cleaning_elves += 1;
                        println!("found a double{}", line)
                    }
                }
            }
        }
    }
    println!("fully contained: {}", double_cleaning_elves)
}

fn get_assigned_sections_for_pair(line: &str) -> (Vec<u32>, Vec<u32>) {
    let elf_string_pairs = line.split(',').collect::<Vec<&str>>();

    let first_elf = elf_string_pairs[0];
    let second_elf = elf_string_pairs[1];

    let first_number_first_elf: u32 = first_elf.split('-').collect::<Vec<&str>>()[0]
        .parse()
        .unwrap();
    let second_number_first_elf: u32 = first_elf.split('-').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    let first_number_second_elf: u32 = second_elf.split('-').collect::<Vec<&str>>()[0]
        .parse()
        .unwrap();
    let second_number_second_elf: u32 = second_elf.split('-').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    let mut first_elf_sections: Vec<u32> = Vec::new();
    let mut second_elf_sections: Vec<u32> = Vec::new();

    for i in first_number_first_elf..second_number_first_elf + 1 {
        first_elf_sections.push(i);
    }

    for i in first_number_second_elf..second_number_second_elf + 1 {
        second_elf_sections.push(i);
    }

    return (first_elf_sections, second_elf_sections);
}

fn elves_fully_contained(first_elf: Vec<u32>, second_elf: Vec<u32>) -> bool {
    let first_elf_set: HashSet<u32> = first_elf.iter().cloned().collect();
    let second_elf_set: HashSet<u32> = second_elf.iter().cloned().collect();

    let first_elf_contains_second = first_elf_set.is_superset(&second_elf_set);
    let second_elf_contains_first = second_elf_set.is_superset(&first_elf_set);

    return first_elf_contains_second || second_elf_contains_first;
}
