use std::collections::HashMap;
use std::fs;

fn main() {
    let file_input = fs::read_to_string("./day1/input.txt");

    let file_string = file_input.expect("Error reading input file");

    let lines_in_file = file_string.split("\n");
    let mut left_column = vec![];
    let mut right_column = vec![];
    let mut distances = vec![];

    for split in lines_in_file {
        let  trimmed_line = split.trim();
        if trimmed_line.is_empty() {
            continue
        }

        let contained_numbers:Vec<String> = trimmed_line.split_whitespace().map(str::to_string)
            .collect();
        let left_number = &contained_numbers[0];
        let right_number = &contained_numbers[1];

        left_column.push(left_number.parse::<i32>().expect("Error parsing number"));
        right_column.push(right_number.parse::<i32>().expect("Error parsing number"));
    }
    left_column.sort();
    right_column.sort();

    if left_column.len() != right_column.len() {
        println!("The columns need to be equal")
    }

    for (i, v) in left_column.iter().enumerate() {
       distances.push(v.abs_diff(right_column[i]));
    }
    println!("Distance is {}",distances.iter().copied().sum::<u32>());


    let mut similarity_score = 0;

    let mut cached_numbers = HashMap::new();

    for i in right_column {
        if let Some(found_number) = cached_numbers.get_mut(&i) {
            *found_number += 1;
        } else {
            cached_numbers.insert(i, 1);
        }
    }

    for i in left_column {
        if let Some(found_number) = cached_numbers.get_mut(&i) {
            similarity_score += i * *found_number;
        }
    }

    println!("Similarity score is {}", similarity_score);
}
