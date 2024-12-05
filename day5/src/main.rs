use std::collections::HashMap;
use std::fs;
use std::ops::Index;
use rand::Rng;

fn main() {
    let content = fs::read_to_string("./day5/input.txt").unwrap();

    let split = content.split("\n\n").collect::<Vec<&str>>();

    let rules = split[0].split("\n")
        .map(|x|x.trim().split("|").collect())
        .collect::<Vec<Vec<&str>>>();

    let rows = split[1].split("\n").map(|x|x.trim().split(",").map(|x|x.parse::<i32>().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>();

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    // put rules in map
    for rule in rules {
        let rule_number = rule[0].parse::<i32>().unwrap();
        let rule_value_to_add = rule[1].parse::<i32>().unwrap();
        if map.contains_key(&rule_number) {
            let mut rule_values = map.get(&rule_number).unwrap().to_vec();
            rule_values.push(rule_value_to_add);
            map.insert(rule_number, rule_values);
        } else {
            map.insert(rule_number, vec![rule_value_to_add]);
        }
    }

    let mut middle_number_sum = 0;

    rows.iter().for_each(|row| {
        let mut checked_row = check_row(row, &map);
        if !checked_row.contains(&false) {
            // solution 1
            //middle_number_sum += row[row.len()/2];
        } else {
            // Fix the ordering
            let mut fixed_row = row.clone();
            while checked_row.iter().position(|x| *x == false).is_some() {
                let index_of_false = checked_row.iter().position(|x| *x == false).unwrap();
                fixed_row = fix_array(&fixed_row, index_of_false);
                checked_row = check_row(&fixed_row, &map);
                if !checked_row.contains(&false) {
                    middle_number_sum += fixed_row[fixed_row.len()/2];
                    println!("Fixed row: {:?}", fixed_row);
                    break;
                }
            }
        }
    });
    println!("Middle number sum: {}", middle_number_sum);
}


fn fix_array(row: &Vec<i32>, index_of_false: usize) -> Vec<i32> {
    let mut row_copy = row.clone();
    row_copy.swap(index_of_false, index_of_false - 1);
    row_copy
}


// True = correct, false = incorrect
fn check_row(row: &Vec<i32>, map: &HashMap<i32, Vec<i32>>) -> Vec<bool> {
    let mut seen_row_values = vec![];
    let bool_vals: Vec<bool> = row.into_iter().map(|x|{
        if let Some(rule_numbers) = map.get(&x) {
            let mut copied_seen_row_values = seen_row_values.clone();
            copied_seen_row_values.retain(|y| rule_numbers.contains(&y));

            if copied_seen_row_values.len() != 0 {
                seen_row_values.push(*x);
                return false;
            }
            seen_row_values.push(*x);
            return true;
        }

        seen_row_values.push(*x);
        true
    }).collect();
    bool_vals
}