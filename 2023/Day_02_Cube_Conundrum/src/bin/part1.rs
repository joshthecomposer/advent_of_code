use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = part1(input);
    let duration = start.elapsed();
    dbg!(output);
    println!("Executed in: {:?}", duration);
}

fn part1(input: &str) -> i32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut games: HashMap<i32, bool> = HashMap::new();

    for line in input.lines() {
        let split_up_games: Vec<&str> = line.split(":").collect();

        let game_name = split_up_games[0].trim().split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        println!("{}", game_name);
        let dice_throws : Vec<&str> =
            split_up_games[1].split(";").map(|s| s.trim()).collect();
        games.insert(game_name, true);
        for throw in dice_throws {
            if let Some(value) = games.get(&game_name){
                if !value {
                    break;
                }
            }
            let temp_arr:Vec<&str> = throw.split(",").map(|t| t.trim()).collect();
            for color in temp_arr.iter() {
                let temp_color2 = color.split(" ").collect::<Vec<&str>>()[1];
                if temp_color2.contains("red") {
                    if &color.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap() > &max_red {
                        games.insert(game_name, false);
                        break;
                    }
                }
                if temp_color2.contains("green") {
                    if &color.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap() > &max_green {
                        games.insert(game_name, false);
                        break;
                    }
                }
                if temp_color2.contains("blue") {
                    if &color.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap() > &max_blue {
                        games.insert(game_name, false);
                        break;
                    }
                }
            }
        }
    }
    dbg!(&games);
    let mut result = 0;
    for (k, &v) in games.iter() {
        if v {
            result += k;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 8);
    }
}
