use std::fmt::Write;
use std::fs;
use regex::Regex;

fn main() {

    let input = fs::read_to_string("./day3/input.txt").expect("This should not happen");

    let mut activated_reading = true;
    let mut start = 0;
    let mut end = 0;
    let multiplication = 0;
    let mut current_block = String::new();

    let mut command_string: String = String::new();
    let mut command_slices = vec![];
    for (index, char) in input.chars().enumerate() {
        command_string.write_char(char).unwrap();
        if command_string.contains("do") && index+6 < input.len() &&
            &input[index..index+6] == "on't()" {
            activated_reading = false;
            end = index;
            println!("Command don't is {}", &input[start..end]);
            command_slices.push(current_block.clone());
            current_block = String::new();
            command_string.clear();
            continue
        }

        println!("Command string is {}", command_string);
        if command_string == "do" {
            start = index;
            activated_reading = true;
            println!("Activated reading from {}", index);
        }

        if activated_reading {
            current_block.write_char(char).expect("TODO: panic message");
        }

        if command_string.len() == 2 {
            command_string = command_string[1..].to_string();
        }
        if index == input.len() - 1 && activated_reading {
            command_slices.push(current_block.clone());
        }
    }
    let mut counter = 0;
    for c in command_slices {
        multiply(&c, &mut counter);
    }
    println!("Counter is {}", counter);
}

fn multiply(input: &str, counter: &mut i32) {
    let regex_for_multiplication = Regex::new(r"mul\((\d+),(\d+)\)").expect("This should not \
    happen");
    let single_capture  = Regex::new(r"mul\((\d+),(\d+)\)").expect("This should not happen");
    regex_for_multiplication.find_iter(&input).for_each(|i| {
        single_capture.captures(&i.as_str().trim()).map(|i| {
            println!("Multiplying {} with {}", i.get(1).map(|i| i.as_str()).unwrap(), i.get(2).map(|i| i.as_str()).unwrap());
            let first = i.get(1).map(|i| i.as_str()).unwrap().parse::<i32>().unwrap();
            let second = i.get(2).map(|i| i.as_str()).unwrap().parse::<i32>().unwrap();
            *counter += first * second;
        });
    });
}
