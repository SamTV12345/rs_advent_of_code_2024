use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
enum MathOperations {
    Add,
    Multiply,
}

fn main() {
    let content = fs::read_to_string("./day7/input.txt").unwrap();

    let lines = content.split("\n");
    let mut map: HashMap<i64, Vec<i64>> = HashMap::new();

    let mut line_counter = 0;
    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }
        line_counter += 1;
        let split = line.split(":").collect::<Vec<&str>>();

        let key = split[0].parse::<i64>().unwrap();
        let value = split[1];
        let value = value.trim();

        value.split_whitespace().for_each(|v|{
            let v = v.trim();
            if v.is_empty() {
                return;
            }
            if let Some(found) = map.get_mut(&key) {
                found.push(v.parse::<i64>().unwrap());
            } else {
                map.insert(key, vec![v.parse::<i64>().unwrap()]);
            }
        })
    }

    let mut total_count = 0;
    for (key, value) in map {

        let n = value.len()-1; // Number of bits
        let max_value: u128 = (1 << n) - 1; // Maximum value for n-bit counter
        let mut counter: u128 = 0;
        for _ in 0..(max_value + 1) {
            let binary_string = format!("{:0width$b}", counter, width = n); // Print the counter
            // value in binary
            // format
            let math_ops = calc_ops_from_str(&binary_string);
            if key == calc_math(&value, &math_ops) {
                total_count += key;
                break;
            }
            counter = (counter + 1) & max_value;
        }
    }
    println!("Total count is {}", total_count);
}

fn calc_ops_from_str(ops: &str) -> Vec<MathOperations> {
    let mut math_ops = vec![];
    for op in ops.chars() {
        match op {
            '0' => {
                math_ops.push(MathOperations::Add);
            }
            '1' => {
                math_ops.push(MathOperations::Multiply);
            }
            _ => {
                panic!("Invalid operator");
            }
        }
    }
    math_ops
}


fn calc_math(values: &Vec<i64>, math_ops: &Vec<MathOperations>) -> i64 {
    let mut result = 0;
    for (index, value) in values.iter().enumerate() {
        if index == 0 {
            result = *value;
            continue;
        }
        match math_ops[index-1] {
            MathOperations::Add => {
                result += value;
            }
            MathOperations::Multiply => {
                result *= value;
            }
        }
    }
    result
}