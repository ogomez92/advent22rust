mod file_utils;

fn main() {
    let mut overlappers = 0;
    if let Ok(lines) = file_utils::read_lines("input.txt") {
        for current_line in lines {
            if let Ok(line) = current_line {
                if !line.trim().is_empty() {
                    let (first_elf, second_elf) = get_assigned_sections_for_pair(line.trim());
                    println!("{}", line);
                    if elves_overlap(first_elf, second_elf) {
                        overlappers += 1;
                    }
                }
            }
        }
    }
    println!("Overlappers, stop wasting your time: {}", overlappers)
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

fn elves_overlap(first_elf: Vec<u32>, second_elf: Vec<u32>) -> bool {
    for first_elf_section in first_elf {
        for second_elf_section in &second_elf {
            if first_elf_section == *second_elf_section {
                return true;
            }
        }
    }
    return false;
}
