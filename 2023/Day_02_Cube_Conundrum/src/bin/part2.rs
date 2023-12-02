use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = part2(input);
    let duration = start.elapsed();
    dbg!(output);
    println!("Executed in: {:?}", duration);
}

fn part2(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        let split_up_games: Vec<&str> = line.split(":").collect();

        let game_name = split_up_games[0].trim().split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        println!("{}", game_name);
        let dice_throws : Vec<&str> =
            split_up_games[1].split(";").map(|s| s.trim()).collect();
        // games.insert(game_name, true);
        for throw in dice_throws {
            let temp_arr:Vec<&str> = throw.split(",").map(|t| t.trim()).collect();
            for color in temp_arr.iter() {
                let temp_color2 = color.split(" ").collect::<Vec<&str>>()[1];
                let int = &color.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
                if temp_color2.contains("red") {
                    if &int > &&max_red {
                        max_red = *int;
                    }
                } else if temp_color2.contains("green") {
                    if  &int > &&max_green {
                        max_green = *int;
                    }
                } else if temp_color2.contains("blue") {
                    if &int > &&max_blue {
                        max_blue = *int;
                    }
                }
            }
        }
        result += max_red * max_green * max_blue;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 2286);
    }
}
