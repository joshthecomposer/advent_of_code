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
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d+,\d+\))").unwrap();

    let re2 = Regex::new(r"(?<lhs>\d+),(?<rhs>\d+)").unwrap();
    let mut result: u32 = 0;

    let matches: Vec<&str> = re.find_iter(input).map(|c| c.as_str()).collect();

    let mut should_do = true;

    println!("{:?}", &matches);
    
    for m in matches {
        match m {
            "do()" => {
                should_do = true;
            },
            "don't()" => {
                should_do = false;
            },
            _=> {
                if should_do {
                    let cs = re2.captures(m).unwrap();
                    result += cs.name("lhs").unwrap().as_str().parse::<u32>().unwrap() * cs.name("rhs").unwrap().as_str().parse::<u32>().unwrap();
                }
            }
        }

    }
result

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = find_solution(
"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
);
        assert_eq!(48, result);
    }
}
