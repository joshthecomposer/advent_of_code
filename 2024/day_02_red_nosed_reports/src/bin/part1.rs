use std::time::Instant;

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
        let report:Vec<isize> = l.split_whitespace()
            .filter_map(|n| n.parse().ok() )
            .collect();

        let mut decreased = false;
        let mut increased = false;

        let mut is_safe = true;

        for (i, n) in report.iter().enumerate() {

            if (i + 1) >= report.len() { continue; }

            let change = n - report[i + 1]; 

            if change.abs() > 3 || change.abs() == 0 { 
                is_safe = false;
                break; 
            }

            if change.is_positive() { decreased = true; }
            if change.is_negative() { increased = true; }

            if decreased && increased { 
                is_safe = false;
                break; 
            }
        }
        if is_safe { total_safe += 1; }
    }
   total_safe
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
        assert_eq!(2, result);
    }
}
