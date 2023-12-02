use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = part1(input);
    let duration = start.elapsed();
    dbg!(output);
    println!("Executed in: {:?}", duration);
}

fn part1(input: &str) -> usize {
    let mut position: HashMap<&str, usize> = HashMap::from([("aim", 0), ("height", 0), ("depth", 0)]);
    let lines_vec: Vec<&str> = input.lines().collect();
    for line in lines_vec.iter() {
        let split_line: Vec<&str> = line.split(" ").collect();
        let current_value = split_line[1].parse::<usize>().unwrap();
        let command = split_line[0];

        match command {
            "forward"=> {
                *position.get_mut("height").unwrap() += current_value;
                *position.get_mut("depth").unwrap() += position["aim"] * current_value;
            }
            "up" => *position.get_mut("aim").unwrap() -= current_value,
            "down"=> *position.get_mut("aim").unwrap() += current_value,
            _=> panic!("Unexpected command detected"),
        }
    }
    position["height"] * position["depth"]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2",
        );
        assert_eq!(result, 900);
    }
}
