use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = part1(input);
    let duration = start.elapsed();
    dbg!(output);
    println!("Executed in: {:?}", duration);
}

fn part1(input: &str) -> i32 {
    let lines_vec : Vec<&str> = input.lines().collect();
    let mut result = 0;
    for (index, line) in lines_vec.iter().enumerate() {
        if index < 1 {
            continue;
        }
        if line.parse::<usize>().unwrap() > lines_vec[index-1].parse::<usize>().unwrap() {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "199
200
208
210
200
207
240
269
260
263",
        );
        assert_eq!(result, 7);
    }
}
