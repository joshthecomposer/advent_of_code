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
    let mut lines_vec: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let mut lines_by_beginning: HashMap<&str, Vec<String>> =
        HashMap::from([("1", vec![]), ("0", vec![])]);
        let mut cursor = 0;
    while lines_vec.len() > 1 && cursor < lines_vec[0].len() {
        for line in lines_vec.iter() {
            match &line[cursor..cursor + 1] {
                "1" => lines_by_beginning.get_mut("1").unwrap().push(line.clone()),
                "0" => lines_by_beginning.get_mut("0").unwrap().push(line.clone()),
                _ => panic!("Something went wronk"),
            };

           
        }
        if lines_by_beginning.get("0").unwrap().len() >= lines_by_beginning.get("1").unwrap().len()
        {
            lines_vec = lines_by_beginning.get("0").unwrap().to_vec();
        } else {
            lines_vec = lines_by_beginning.get("1").unwrap().to_vec();
        }
        cursor += 1;
    }
    dbg!(lines_by_beginning);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );
        assert_eq!(result, 230);
    }
}
