fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut result = 0;
    let data : Vec<Vec<&str>> = input.lines().map(|s| s.split(" ").collect()).collect();

    for line in data {
        match line[1] {
            "X" => {
                result += 1;
                match line[0] {
                    "A"=> result += 3,
                    "B" => result += 0,
                    "C" => result += 6,
                    _=> ()
                }
            }
            "Y" => {
                result += 2;
                match line[0] {
                    "A"=> result += 6,
                    "B" => result += 3,
                    "C" => result += 0,
                    _=> (),
                }
            }
            "Z" => {
                result += 3;
                match line[0] {
                    "A"=> result += 0,
                    "B" => result += 6,
                    "C" => result += 3,
                    _=> (),
                }
            }
            _=> ()
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
            "A Y
B X
C Z
",
        );
        assert_eq!(15, result);
    }
}

