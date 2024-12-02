use std::fs;

fn main() {
    let input = fs::read_to_string("./day2/input.txt").expect("This should not happen");

    let lines = input.split("\n");
    let mut counter = 0;

    for line in lines {
        let line_numbers: Vec<i32> = line.split_whitespace().map(|i|i.trim().parse::<i32>()
            .expect("This should be all numbers"))
            .collect();
        if is_safe_level(&line_numbers) {
            counter += 1;
        }
        else {
            // Try to repair the levels
            for i in 0..line_numbers.len() {
                let mut new_level = line_numbers.clone();
                new_level.remove(i);
                if is_safe_level(&new_level) {
                    counter += 1;
                    break;
                }
            }
        }
    }
    println!("Safe levels are {}", counter);
}


fn is_safe_level(level: &Vec<i32>) -> bool {
    let increasing = level.windows(2).all(|i| i[0] < i[1]);
    let decreasing = level.windows(2).all(|i| i[0] > i[1]);

    if increasing || decreasing {
        // Check adjacent levels
        let level_diff_ok = level.windows(2).all(|i| i[0].abs_diff(i[1]) <=3 && i[0]
            .abs_diff(i[1]) >= 1);
        if level_diff_ok {
            return true
        }
        return false
    }
    false
}