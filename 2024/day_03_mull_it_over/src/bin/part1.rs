use std::time::Instant;
use regex::{Match, Regex};

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = find_solution(input);
    let duration = start.elapsed();
    dbg!(output);
    println!("Executed in: {:?}", duration);
}

fn find_solution(input: &str) -> u32 {
    let re = Regex::new(r"(mul)\((?<lhs>\d+),(?<rhs>\d+)\)").unwrap();

    let sum = re.captures_iter(input)
        .map(|c| c["lhs"].parse::<u32>().unwrap() * c["rhs"].parse::<u32>().unwrap() )
        .sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = find_solution(
"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
);
        assert_eq!(161, result);
    }
}
