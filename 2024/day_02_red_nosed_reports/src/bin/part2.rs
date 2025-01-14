use std::{any::Any, time::Instant};

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = find_solution(input);
    let duration = start.elapsed();
    dbg!(output);
    println!("Executed in: {:?}", duration);
}

fn find_solution(input: &str) -> isize {
    let mut total_safe = 0;

    for l in input.lines() {
        let mut report:Vec<isize> = l.split_whitespace()
            .filter_map(|n| n.parse().ok() )
            .collect();

        if is_safe(&report) {
            total_safe += 1;
            continue;
        }

        for i in 0..report.len() {
            let n = report.remove(i);

            if is_safe(&report) {
                total_safe += 1;
                break;
            }

            report.insert(i, n);
        }
    }
   total_safe
}

fn is_safe(input: &Vec<isize>) -> bool {
        let mut decreased = false;
        let mut increased = false;

        for (i, n) in input.iter().enumerate() {

            if (i + 1) >= input.len() { continue; }

            let change = n - input[i + 1]; 

            if change.abs() > 3 || change.abs() == 0 { 
                return false;
            }

            if change.is_positive() { decreased = true; }
            if change.is_negative() { increased = true; }

            if decreased && increased { 
                return false;
            }
        }
        true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = find_solution(
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
);
        assert_eq!(4, result);
    }
}
