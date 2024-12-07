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

fn find_solution(input: &str) -> i32 {
    let re = Regex::new(r"(mul)\(\d+,\d+\)").unwrap();
    let re2 = Regex::new(r"\d+,\d+").unwrap();
    let mut result = 0;

    let matches: Vec<&str> = re.find_iter(input)
        .map(|m| m.as_str())
        .collect();
    
    for m in matches {
        let nums: Vec<i32> = re2.find(m)
            .unwrap()
            .as_str()
            .split(",")
            .filter_map(|n| n.parse().ok())
            .collect();

        result += nums[0] * nums[1];
    }
    result
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
