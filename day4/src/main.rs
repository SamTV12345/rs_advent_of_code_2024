use std::fs;
use regex::Regex;

fn main() {
    let file_content = fs::read_to_string("./day4/input.txt").unwrap();
    let lines: Vec<&str> = file_content.split('\n').collect();
    let mut matrix: Vec<Vec<char>> = vec![vec![]; lines.len()];

    for (index, line) in lines.iter().enumerate() {
        line.chars().for_each(|c| {
            matrix[index].push(c);
        });
    }

    for row in matrix.iter() {
        println!("{:?}", row);
    }
    check_for_x(&matrix);
}

fn check_for_x(matrix: &Vec<Vec<char>>) {

    let n = matrix.len();
    let m = matrix[0].len();

    let mut counter = 0;
    for r in 0..n {
        for c in 0..m {
            if matrix[r][c] == 'A' {
                let u_r = r as i32;
                let u_c = c as i32;
                if check_if_in_range_of_matrix(&matrix, u_r-1, u_c-1) && matrix[r-1][c-1] == 'M'
                    && check_if_in_range_of_matrix(&matrix, u_r-1,u_c+1) && matrix[r-1][c+1] == 'M'
                    && check_if_in_range_of_matrix(&matrix, u_r+1,u_c-1) && matrix[r+1][c-1] == 'S'
                    && check_if_in_range_of_matrix(&matrix, u_r+1,u_c+1) && matrix[r+1][c+1] == 'S' {
                    counter += 1;
                }

                if check_if_in_range_of_matrix(&matrix, u_r-1, u_c-1) && matrix[r-1][c-1] == 'S'
                    && check_if_in_range_of_matrix(&matrix, u_r-1,u_c+1) && matrix[r-1][c+1] == 'S'
                    && check_if_in_range_of_matrix(&matrix, u_r+1,u_c-1) && matrix[r+1][c-1] == 'M'
                    && check_if_in_range_of_matrix(&matrix, u_r+1,u_c+1) && matrix[r+1][c+1] == 'M' {
                    counter += 1;
                }

                if check_if_in_range_of_matrix(&matrix, u_r-1, u_c-1) && matrix[r-1][c-1] == 'S'
                    && check_if_in_range_of_matrix(&matrix, u_r-1,u_c+1) && matrix[r-1][c+1] == 'M'
                    && check_if_in_range_of_matrix(&matrix, u_r+1,u_c-1) && matrix[r+1][c-1] == 'S'
                    && check_if_in_range_of_matrix(&matrix, u_r+1,u_c+1) && matrix[r+1][c+1] == 'M' {
                    counter += 1;
                }

                if check_if_in_range_of_matrix(&matrix, u_r-1, u_c-1) && matrix[r-1][c-1] == 'M'
                    && check_if_in_range_of_matrix(&matrix, u_r-1,u_c+1) && matrix[r-1][c+1] == 'S'
                    && check_if_in_range_of_matrix(&matrix, u_r+1,u_c-1) && matrix[r+1][c-1] == 'M'
                    && check_if_in_range_of_matrix(&matrix, u_r+1,u_c+1) && matrix[r+1][c+1] == 'S' {
                    counter += 1;
                }
            }
        }
    }
    println!("X-X MAS counter is {}", counter);
}


fn check_if_in_range_of_matrix(matrix: &Vec<Vec<char>>, row: i32, column: i32) -> bool {
    let n = matrix.len() as i32;
    let m = matrix[0].len() as i32;
    if row >= 0 && row < n && column >= 0 && column < m {
        return true;
    }
    false
}

fn search_word_in_matrix(matrix: &Vec<Vec<char>>, word: &str) -> i32 {
    let mut total_results = 0;

    // Horizontal search
    /*for row in matrix {
        total_results += scan_string(&row.iter().collect::<String>(), word);
    }*/

    // Vertical search

    /*for column in 0..matrix[0].len() {
        let mut column_string = String::new();
        for row in matrix {
            column_string.push(row[column]);
        }
        total_results += scan_string(&column_string, word);
    }*/

    // Diagonal search (top-left to bottom-right)
    let n = matrix.len();
    let m = matrix[0].len();
    for d in 0..(n + m - 1) {
        let mut diagonal = String::new();
        for i in 0..n {
            let j = i as isize + d as isize - (n as isize - 1);
            if j >= 0 && j < m as isize {
                diagonal.push(matrix[i][j as usize]);
            }
        }
        if !diagonal.is_empty() {
            total_results += scan_string(&diagonal, word);
        }
    }

    // Diagonal search (top-right to bottom-left)
    for d in 0..(n + m - 1) {
        let mut diagonal = String::new();
        for i in 0..n {
            let j = d as isize - i as isize;
            if j >= 0 && j < m as isize {
                diagonal.push(matrix[i][j as usize]);
            }
        }
        if !diagonal.is_empty() {
            total_results += scan_string(&mut diagonal, word);
        }
    }
    total_results
}

fn scan_string(input: &str, word: &str) -> i32 {
    let mut counter = 0;


    counter
}