use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = find_solution(input);
    let duration = start.elapsed();
    dbg!(output);
    println!("Executed in: {:?}", duration);
}

fn find_solution(input: &str) -> i64 {
    let mut distance = 0;
    let mut left_numbers:Vec<i64> = vec![];
    let mut right_numbers:Vec<i64> = vec![];

    for l in input.lines() {
        let row: Vec<&str> = l.split_whitespace().collect();
        left_numbers.push(row[0].parse().unwrap());
        right_numbers.push(row[1].parse().unwrap());
    };

    left_numbers.sort_unstable();
    right_numbers.sort_unstable();

    for (i, nl) in left_numbers.iter().enumerate() {
        let nr = right_numbers[i];

        distance += (nl - nr).abs();
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = find_solution(
"3   4
4   3
2   5
1   3
3   9
3   3" 
        );
        assert_eq!(11, result);
    }
}
