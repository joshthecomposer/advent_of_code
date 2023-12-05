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
    let lines_vec = input.lines().collect::<Vec<&str>>();
    /*
    hash map:
    {
        Card 1: [[41,48,83,86,17], [83, 86,  6, 31, 17,  9, 48, 53]]
    }
    there is white space in some of the numbers so that they line up... figure out how to trim this off when processing.
     */
    let mut hash_map: HashMap<&str, Vec<Vec<usize>>> = HashMap::new();
    //Parse input into the hashmap above.
    for line in lines_vec.iter() {
        let split1 = line.split(":").collect::<Vec<&str>>();
        let key = split1[0];
        let value = split1[1]
            .split("|")
            .map(|s1| {
                s1.split_whitespace()
                    .map(|s2| s2.trim().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        hash_map.insert(key, value);
    }

    let mut result: usize = 0;

    for (key, _) in &hash_map {
        let mut current_counter = 0;
        for winning in &hash_map[key][0] {
            for pick in &hash_map[key][1] {
                if winning == pick {
                    if current_counter < 1 {
                        current_counter += 1;
                    } else {
                        current_counter *= 2;
                    }
                }
            }
        }
        result += current_counter;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(30, result);
    }
}
