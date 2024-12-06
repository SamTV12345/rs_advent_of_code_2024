use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct GuardPosition {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct ObstaclePosition {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum ObstacleType {
    Solid,
    Obstacle,
    Empty
}


#[derive(Debug)]
struct ObstacleError;

fn parse(input: &str) -> (HashMap<ObstaclePosition, ObstacleType>, GuardPosition ) {
    let mut obstacles = HashMap::new();
    let mut guard_position = GuardPosition {
        x: 0,
        y: 0,
        direction: Direction::Up,
    };

    for (index_y, row) in input.split("\n").enumerate() {
        let row = row.trim();
        if row.is_empty() {
            continue;
        }
        row.chars().enumerate().for_each(|(index_x, c)| {
            if c == '#' {
                obstacles.insert(
                    ObstaclePosition {
                        x: index_x as i32 +1,
                        y: index_y as i32+1,
                    },
                    ObstacleType::Obstacle,
                );
            } else if c == '^' {
                guard_position = GuardPosition {
                    x: index_x as i32 +1,
                    y: index_y as i32 +1,
                    direction: Direction::Up,
                };
            } else {
                obstacles.insert(
                    ObstaclePosition {
                        x: index_x as i32 +1,
                        y: index_y as i32 +1,
                    },
                    ObstacleType::Empty,
                );
            }
        })
    }

    (obstacles, guard_position)
}

fn run_code(obstacle_input: &str) -> HashSet<ObstaclePosition> {

    let (mut obstacles, mut guard_position) = parse(obstacle_input);
    let mut visited_fields: HashSet<ObstaclePosition> = HashSet::new();
    obstacles.insert(
        ObstaclePosition {
            x: guard_position.x,
            y: guard_position.y,
        },
        ObstacleType::Empty,
    );

    let start_position = ObstaclePosition{
        y: guard_position.y,
        x: guard_position.x,
    };

    while let Some(_) = obstacles.get(&ObstaclePosition {
        x: guard_position.x,
        y: guard_position.y,
    }) {
        let next_field = calculate_next_step(&guard_position);
        let result = move_to_position(&next_field, &obstacles, &mut guard_position);
        if let Ok(next_field) = result {
            visited_fields.insert(next_field);
        } else {
            break;
        }
    }
    visited_fields
}


fn main() {
    let content = fs::read_to_string("./day6/sample.txt").unwrap();
    let visited_fields = run_code(&content);
    let (obstacles, guard_position) = parse(&content);
    let start_position = ObstaclePosition {
        x: guard_position.x,
        y: guard_position.y,
    };
    //println!("Visited fields: {}", visited_fields.len());

    let mut loop_fields = HashSet::new();

    for position in visited_fields {
        if start_position != position {
            if place_obstacle_at_position(obstacles.clone(), position.clone(), start_position.clone(),
                                          &guard_position.direction) {
                loop_fields.insert(position);
            } else {
                println!("No loop")
            }
        }
    }

    println!("Loops: {}", loop_fields.len());
}


fn place_obstacle_at_position(
    mut obstacles: HashMap<ObstaclePosition, ObstacleType>,
    current_position_to_place: ObstaclePosition,
    start_position: ObstaclePosition,
    guard_direction: &Direction,) -> bool {

    if let Some(cur_pos) = obstacles.get_mut(&current_position_to_place) {
        // Set it to be solid
        *cur_pos = ObstacleType::Solid;
        return loop_guard(GuardPosition {
            x: start_position.x,
            y: start_position.y,
            direction: guard_direction.clone(),
        }, obstacles.clone())
    }
    false
}

fn move_to_position(next_field: &ObstaclePosition, obstacles: &HashMap<ObstaclePosition, ObstacleType>,
                    guard_position: &mut GuardPosition) ->
                                                                         Result<ObstaclePosition, ObstacleError> {
    if let Some(is_obstacle) = obstacles.get(&next_field) {
        if *is_obstacle == ObstacleType::Obstacle || *is_obstacle == ObstacleType::Solid {
            match guard_position.direction {
                Direction::Up => {
                    guard_position.direction = Direction::Right;
                    move_to_position(&calculate_next_step(guard_position), obstacles,
                                      guard_position)
                }
                Direction::Down => {
                    guard_position.direction = Direction::Left;
                    move_to_position(&calculate_next_step(guard_position), obstacles,
                                     guard_position)
                }
                Direction::Left => {
                    guard_position.direction = Direction::Up;
                    move_to_position(&calculate_next_step(guard_position), obstacles,
                                     guard_position)
                }
                Direction::Right => {
                    guard_position.direction = Direction::Down;
                    move_to_position(&calculate_next_step(guard_position), obstacles,
                                     guard_position)
                }
            }
        } else {
            guard_position.x = next_field.x;
            guard_position.y = next_field.y;
            let cloned_next_field = next_field.clone();
            Ok(cloned_next_field)
        }
    } else {
        Err(ObstacleError{})
    }
}

fn loop_guard(mut guard_position: GuardPosition, obstacles:
HashMap<ObstaclePosition, ObstacleType>) -> bool {
    let start_position = ObstaclePosition {
        x: guard_position.x,
        y: guard_position.y,
    };
    let start_direction = guard_position.direction.clone();

    // Move one position so it doesn't exit immediately
    let obstacle_position = calculate_next_step(&guard_position);
    let next_position = move_to_position(&obstacle_position, &obstacles, &mut guard_position);

    if let Err(_) = next_position {
        return false;
    }


    let mut counter = 0;
    while guard_position.y != start_position.y || guard_position.x != start_position.x ||
        guard_position.direction != start_direction {
        counter += 1;
        println!("Counter: {}", counter);
        let obstacle_position_next = calculate_next_step(&guard_position);
        let next_position = move_to_position(&obstacle_position_next, &obstacles, &mut
            guard_position);
        if next_position.is_err() {
            return false
        }
    }
    println!("Looped");
    true
}

fn calculate_next_step(guard_position: &GuardPosition ) -> ObstaclePosition {
    let next_field;
    match guard_position.direction {
        Direction::Up => {
            next_field = ObstaclePosition {
                x: guard_position.x.clone(),
                y: guard_position.y - 1,
            }
        }
        Direction::Down => {
            next_field = ObstaclePosition {
                x: guard_position.x.clone(),
                y: guard_position.y + 1,
            }
        }
        Direction::Left => {
            next_field = ObstaclePosition {
                x: guard_position.x.clone() - 1,
                y: guard_position.y,
            }
        }
        Direction::Right => {
            next_field = ObstaclePosition {
                x: guard_position.x.clone() + 1,
                y: guard_position.y,
            }
        }
    }
    next_field
}



#[cfg(test)]
mod tests {
    use crate::run_code;

    const SAMPLE: &str = r#"
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
    "#;
    #[test]
    fn solve_puzzle_1() {
        let result = run_code(SAMPLE);
        assert_eq!(result.len(), 41)
    }

    #[test]
    fn solve_puzzle_1_real_input() {
        let content = std::fs::read_to_string("./input.txt").unwrap();
        let result = run_code(&content);
        assert_eq!(result.len(), 5329)
    }
}
