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
    let lines_vec: Vec<&str> = input.lines().collect();
    let line_length = lines_vec.first().expect("Input should not be empty").len();
    let mut final_values: HashMap<&str, usize> = HashMap::from([("gamma", 0), ("epsilon", 0)]);
    let mut final_strs_to_convert: HashMap<&str, String> =
        HashMap::from([("gamma", "".to_string()), ("epsilon", "".to_string())]);
    for cursor in 0..line_length {
        let mut positions_and_values: HashMap<&str, usize> = HashMap::from([("0", 0), ("1", 0)]);
        // dbg!(cursor);
        for (_index, _line) in lines_vec.iter().enumerate() {
            // dbg!(_index);
            //I have read that as_bytes is dangerous because non-ascii chars can be more than one byte.
            //However, we are expecting only 0's and 1's so it makes sense here.
            match _line.as_bytes()[cursor] {
                b'0' => *positions_and_values.get_mut("0").unwrap() += 1,
                b'1' => *positions_and_values.get_mut("1").unwrap() += 1,
                _ => panic!("Something went wrong"),
            }
        }
        if positions_and_values["0"] > positions_and_values["1"] {
            final_strs_to_convert
                .get_mut("gamma")
                .unwrap()
                .push_str("0");
            final_strs_to_convert
                .get_mut("epsilon")
                .unwrap()
                .push_str("1");
        } else {
            final_strs_to_convert
                .get_mut("gamma")
                .unwrap()
                .push_str("1");
            final_strs_to_convert
                .get_mut("epsilon")
                .unwrap()
                .push_str("0");
        }
    }
    *final_values.get_mut("gamma").unwrap() +=
        usize::from_str_radix(final_strs_to_convert.get_mut("gamma").unwrap(), 2).unwrap();
    *final_values.get_mut("epsilon").unwrap() +=
        usize::from_str_radix(final_strs_to_convert.get_mut("epsilon").unwrap(), 2).unwrap();
    dbg!(&final_values);
    final_values["gamma"] * final_values["epsilon"]
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
        assert_eq!(result, 198);
    }
}
