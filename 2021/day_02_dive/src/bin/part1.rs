use std::{time::Instant, collections::HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = part1(input);
    let duration = start.elapsed();
    dbg!(output);
    println!("Executed in: {:?}", duration);
}

fn part1(input: &str) -> usize {
    let mut position:HashMap<&str, usize> = HashMap::from([
        ("h", 0),
        ("d", 0),
    ]);
    let lines_vec: Vec<&str> = input.lines().collect();
    for line in lines_vec.iter() {
        let split_line: Vec<&str> = line.split(" ").collect();
        let command = split_line[0];
        let value = split_line[1].parse::<usize>().unwrap();

        match command {
            "forward"=> *position.get_mut("h").unwrap() += value,
            "up"=> *position.get_mut("d").unwrap() -= value,
            "down"=> *position.get_mut("d").unwrap() += value,
            _=> panic!("Invalid command detected"),
        }
    }
    position["h"] * position["d"]
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
        assert_eq!(result, 150);
    }
}
