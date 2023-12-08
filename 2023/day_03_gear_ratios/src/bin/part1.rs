use std::time::Instant;

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
    let mut result = 0;
    const DOT:char = '.';

    for (y, line) in lines_vec.iter().enumerate() {
        let mut found_a_gear = false;
        let mut current_number_string = "".to_string();
        for (x, c) in line.char_indices() {
            if c.is_numeric() {
                current_number_string.push(c);
                //search vertically
                if y > 0 && y < lines_vec.len() - 1 {
                    //we are in the middle of the block of arrays vertically, search up and down.
                    let previous_line = &lines_vec[y - 1];
                    let character_above = previous_line.chars().nth(x).unwrap();

                    let next_line = &lines_vec[y + 1];
                    let character_below = next_line.chars().nth(x).unwrap();

                    if !character_above.is_numeric() && character_above != DOT{
                        found_a_gear = true;
                    }
                    if !character_below.is_numeric() && character_below != DOT {
                        found_a_gear = true;
                    }
                } else if y > 0 {
                    // we are at the bottom, search up.
                    let previous_line = &lines_vec[y - 1];
                    let character_above = previous_line.chars().nth(x).unwrap();

                    if !character_above.is_numeric() && character_above != DOT {
                        found_a_gear = true;
                    }
                } else {
                    // we are at the top, search down
                    let next_line = &lines_vec[y + 1];
                    let character_below = next_line.chars().nth(x).unwrap();

                    if !character_below.is_numeric() && character_below != DOT {
                        found_a_gear = true;
                    }
                }
                //search horizontally
                if x > 0 && x < line.len () - 1 {
                    //we are in the middle of the array horizontally, search left and right
                    let left_char = line.chars().nth(x - 1).unwrap();
                    let right_char = line.chars().nth(x + 1).unwrap();

                    if (!left_char.is_numeric() && left_char != DOT)
                        || (!right_char.is_numeric() && right_char != DOT)
                    {
                        found_a_gear = true;
                    }
                } else if x > 0 && x == line.len() - 1 {
                    //we are at the end, search left only.
                    let left_char = line.chars().nth(x - 1).unwrap();

                    if !left_char.is_numeric() && left_char != DOT {
                        found_a_gear = true;
                    }
                } else if x == 0 {
                    //we are at the beginning, search right only.
                    let right_char = line.chars().nth(x + 1).unwrap();
                    if !right_char.is_numeric() && right_char != DOT {
                        found_a_gear = true;
                    }
                }
                // Check diagonals
                if y + 1 < lines_vec.len() && x < line.len() - 1 {
                    // lower right can be evaluated
                    let down_one = y+1;
                    let right_one = x+1;

                    let lower_right_diag = lines_vec[down_one].chars().nth(right_one).unwrap();

                    if !lower_right_diag.is_numeric() && lower_right_diag != DOT {
                        found_a_gear = true;
                    }
                }
                if y + 1 < lines_vec.len() && x > 0 {
                    // lower left can be evaluated
                    let down_one = y+1;
                    let left_one = x-1;

                    let lower_left_diag = lines_vec[down_one].chars().nth(left_one).unwrap();

                    if !lower_left_diag.is_numeric() && lower_left_diag != DOT {
                        // dbg!(lower_left_diag);
                        found_a_gear = true;
                    }
                }
                if y > 0 && x < line.len() - 1 {
                    // upper right can be evaluated
                    let up_one = y-1;
                    let right_one = x+1;

                    let lower_right_diag = lines_vec[up_one].chars().nth(right_one).unwrap();

                    if !lower_right_diag.is_numeric() && lower_right_diag != DOT {
                        found_a_gear = true;
                    }
                }
                if  y > 0 && x > 0 {
                    //upper left can be evaluated
                    let up_one = y-1;
                    let left_one = x-1;

                    let lower_left_diag = lines_vec[up_one].chars().nth(left_one).unwrap();

                    if !lower_left_diag.is_numeric() && lower_left_diag != DOT {
                        found_a_gear = true;
                    }
                }
            } else {
                if found_a_gear == true {
                    let to_add = current_number_string.parse::<usize>().unwrap();
                    result += to_add;
                    found_a_gear = false;
                }
                // dbg!(&current_number_string);
                current_number_string = "".to_string();
            }
        }
        if found_a_gear && !current_number_string.is_empty() {
            let to_add = current_number_string.parse::<usize>().unwrap();
            result += to_add;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4419);
    }
}
