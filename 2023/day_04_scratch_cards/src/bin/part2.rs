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
    let mut hash_map: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();

    for line in lines_vec.iter() {
        let split = line.split(":").collect::<Vec<&str>>();
        let key = split[0].split_whitespace().last().unwrap().parse::<usize>().unwrap();
        let value = split[1]
            .split("|")
            .map(|s| s.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect())
            .collect();
        hash_map.insert(key, value);
    }

    let mut card_copies: HashMap<usize, usize> = (1..=hash_map.len()).map(|n| (n, 1)).collect();
    let last_card_number = *hash_map.keys().max().unwrap();

    for card_number in 1..=last_card_number {
        //This is a much better way to store the card data to then iterate over, rather than doing it all at once.
        let card_data = &hash_map[&card_number];
        let winning_numbers = &card_data[0];
        let picked_numbers = &card_data[1];

        let matches = winning_numbers.iter().filter(|&&n| picked_numbers.contains(&n)).count();

        let mut counts_to_add: Vec<(usize, usize)> = Vec::new();
        for next_card in card_number+1..=std::cmp::min(card_number+matches, last_card_number) {
            let count = card_copies.get(&card_number).unwrap_or(&0);
            counts_to_add.push((next_card, *count));
        }

        for (card, count) in counts_to_add {
            *card_copies.entry(card).or_insert(0) += count;
        }
    }

    card_copies.values().sum()
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
        assert_eq!(result, 30);
    }
}
