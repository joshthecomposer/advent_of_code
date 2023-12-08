use std::{time::Instant, collections::{HashMap, HashSet}, fs::File};
use std::io::Write;
type Position = (usize, usize);

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
    const GEAR:char = '*';
    let mut gear_locations: HashMap<Position, Vec<usize>> = HashMap::new();

    for (y, line) in lines_vec.iter().enumerate() {
        let mut found_a_gear = false;
        let mut current_number_string = "".to_string();
        //use a hashset to insure unique values
       let mut positions_to_add:HashSet<Position> = HashSet::new();
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

                    if character_above == GEAR{
                        found_a_gear = true;
                        positions_to_add.insert((x,y-1));
                    }
                    if character_below == GEAR {
                        found_a_gear = true;
                        positions_to_add.insert((x,y+1));
                    }
                } else if y > 0 {
                    // we are at the bottom, search up.
                    let previous_line = &lines_vec[y - 1];
                    let character_above = previous_line.chars().nth(x).unwrap();

                    if character_above == GEAR {
                        found_a_gear = true;
                        positions_to_add.insert((x,y-1));
                    }
                } else {
                    // we are at the top, search down
                    let next_line = &lines_vec[y + 1];
                    let character_below = next_line.chars().nth(x).unwrap();

                    if character_below == GEAR {
                        found_a_gear = true;
                        positions_to_add.insert((x,y+1));
                    }
                }
                //search horizontally
                if x > 0 && x < line.len () - 1 {
                    //we are in the middle of the array horizontally, search left and right
                    let left_char = line.chars().nth(x - 1).unwrap();
                    let right_char = line.chars().nth(x + 1).unwrap();


                    if left_char == GEAR {
                        positions_to_add.insert((x-1, y));
                    }
                    if right_char == GEAR {
                        positions_to_add.insert((x+1, y));
                    }
                } else if x > 0 && x == line.len() - 1 {
                    //we are at the end, search left only.
                    let left_char = line.chars().nth(x - 1).unwrap();

                    if left_char == GEAR {
                        found_a_gear = true;
                        positions_to_add.insert((x-1,y));
                    }
                } else if x == 0 {
                    //we are at the beginning, search right only.
                    let right_char = line.chars().nth(x + 1).unwrap();
                    if right_char == GEAR {
                        found_a_gear = true;
                        positions_to_add.insert((x+1,y));
                    }
                }
                // Check diagonals
                if y + 1 < lines_vec.len() && x < line.len() - 1 {
                    // lower right can be evaluated
                    let down_one = y+1;
                    let right_one = x+1;

                    let lower_right_diag = lines_vec[down_one].chars().nth(right_one).unwrap();

                    if lower_right_diag == GEAR {
                        found_a_gear = true;
                        positions_to_add.insert((x+1,y+1));
                    }
                }
                if y + 1 < lines_vec.len() && x > 0 {
                    // lower left can be evaluated
                    let down_one = y+1;
                    let left_one = x-1;

                    let lower_left_diag = lines_vec[down_one].chars().nth(left_one).unwrap();

                    if lower_left_diag == GEAR {
                        // dbg!(lower_left_diag);
                        found_a_gear = true;
                        positions_to_add.insert((x-1,y+1));
                    }
                }
                if y > 0 && x < line.len() - 1 {
                    // upper right can be evaluated
                    let up_one = y-1;
                    let right_one = x+1;

                    let lower_right_diag = lines_vec[up_one].chars().nth(right_one).unwrap();

                    if lower_right_diag == GEAR {
                        found_a_gear = true;
                        positions_to_add.insert((x+1,y-1));
                    }
                }
                if  y > 0 && x > 0 {
                    //upper left can be evaluated
                    let up_one = y-1;
                    let left_one = x-1;

                    let lower_left_diag = lines_vec[up_one].chars().nth(left_one).unwrap();

                    if lower_left_diag == GEAR {
                        found_a_gear = true;
                        positions_to_add.insert((x-1,y-1));
                    }
                }
            } else {
                if found_a_gear {
                    let to_multiply = current_number_string.parse::<usize>().unwrap();
                    for p in positions_to_add.iter() {
                        if !gear_locations.entry(*p).or_insert(Vec::new()).contains(&to_multiply) {
                            gear_locations.get_mut(p).unwrap().push(to_multiply);
                        }
                    }
                    positions_to_add.clear();
                    found_a_gear = false;
                }
                current_number_string = "".to_string();
            }
        }
        if found_a_gear && !current_number_string.is_empty() {
            let to_multiply = current_number_string.parse::<usize>().unwrap();
            for p in positions_to_add.iter() {
                gear_locations.entry(*p).or_insert(Vec::new()).push(to_multiply);
            }
        }
    }
    // dbg!(&gear_locations);
    for (_, value) in &gear_locations {

        if value.len() == 2 {
            result += value[0] * value[1]
        }
    }
    if let Err(e) = write_gear_locations_to_file(&gear_locations, "output.txt") {
        eprintln!("Failed to write to file: {}", e);
    }
    result
}

fn write_gear_locations_to_file(gear_locations: &HashMap<Position, Vec<usize>>, file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;

    for (position, values) in gear_locations {
        writeln!(file, "Position: {:?}, Values: {:?}", position, values)?;
    }

    Ok(())
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
617*60....
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 504855);
    }
}
