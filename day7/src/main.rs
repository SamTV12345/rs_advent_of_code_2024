use std::collections::HashMap;
use std::fs;
use rayon::prelude::*;

#[derive(Clone, Debug)]
enum MathOperations {
    Add,
    Multiply,
}

fn main() {
    let content = fs::read_to_string("./day7/input.txt").unwrap();

    fn parse_line(line: &str) -> (u64, Vec<u64>) {
        let (eq_result, eq_operands) = line.split_once(':').unwrap();
        (
            eq_result.parse().unwrap(),
            eq_operands
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap()).collect(),
        )
    }


    let total_count: u64 = content
        .par_lines()
        .filter_map(|line| {
            let (eq_result, eq_operands) = parse_line(line);
            can_be_solved_number_1(eq_result, eq_operands, 0).then_some(eq_result)
        })
        .sum();

    let total_count_2: u64 = content
        .par_lines()
        .filter_map(|line| {
            let (eq_result, eq_operands) = parse_line(line);
            can_be_solved_numer_2(eq_result, eq_operands, 0).then_some(eq_result)
        })
        .sum();

    println!("Total count is {}", total_count);
    println!("Total count 2 is {}", total_count_2);
}

fn can_be_solved_number_1(key: u64, values: Vec<u64>, current_result: u64) -> bool {
    if let Some(val) = values.get(0) {
        can_be_solved_number_1(key, values[1..].to_vec(), current_result + val)
            || can_be_solved_number_1(key, values[1..].to_vec(), current_result * val)
    } else {
        key == current_result
    }
}

fn can_be_solved_numer_2(key: u64, values: Vec<u64>, current_result: u64) -> bool {
    if let Some(val) = values.get(0) {
        can_be_solved_numer_2(key, values[1..].to_vec(), current_result + val)
            || can_be_solved_numer_2(key, values[1..].to_vec(), current_result * val)
            // 72||10 = 7210 ist eigentlich 72 * 10^(Anzahl an Stellen von 10) + 10
            || can_be_solved_numer_2(key, values[1..].to_vec(), current_result * u64::pow(10, val
            .to_string().len() as u32) + val)
    } else {
        key == current_result
    }
}