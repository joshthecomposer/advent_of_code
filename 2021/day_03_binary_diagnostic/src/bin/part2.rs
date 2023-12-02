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
    let lines_vec: Vec<&str> = input.lines().collect();
    let mut result = 0;
    let mut prev = 0;
    let mut first_iter = true;
    for i in 0..lines_vec.len() {
        if i + 2 > lines_vec.len() - 1 {
            break;
        }
        let temp = lines_vec[i].parse::<usize>().unwrap()
            + lines_vec[i + 1].parse::<usize>().unwrap()
            + lines_vec[i + 2].parse::<usize>().unwrap();
        if first_iter {
            first_iter = false;
            continue;
        }
        if temp > prev {

            result += 1;
        }
        prev = temp;
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
        assert_eq!(result, 5);
    }
}