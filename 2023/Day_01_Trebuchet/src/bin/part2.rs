fn main() {
    let input = include_str!("./input1.txt");
    let converted = part2(input);
    let result = part1(&converted);
    dbg!(result);
}

fn part1(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let mut temp = String::from("");
        for c in line.chars() {
            if c.is_numeric() {
                temp.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                temp.push(c);
                break;
            }
        }
        total += temp.parse::<i32>().unwrap();
    }
    total.to_string()
}

fn part2(input: &str) -> String {
    let mut new_lines = "".to_string();
    for line in input.lines() {
        for i in 0..line.len() {
            let subst = &line[i..];
            if subst.starts_with("one") {
                new_lines.push_str("1");
            } else if subst.starts_with("two") {
                new_lines.push_str("2");
            } else if subst.starts_with("three") {
                new_lines.push_str("3");
            } else if subst.starts_with("four") {
                new_lines.push_str("4");
            } else if subst.starts_with("five") {
                new_lines.push_str("5");
            } else if subst.starts_with("six") {
                new_lines.push_str("6");
            } else if subst.starts_with("seven") {
                new_lines.push_str("7");
            } else if subst.starts_with("eight") {
                new_lines.push_str("8");
            } else if subst.starts_with("nine") {
                new_lines.push_str("9");
            } else {
                new_lines.push(subst.chars().next().unwrap())
            }
        }
        new_lines += "\n";
    }
    new_lines
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let converted = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
        );
        let result = part1(&converted);
        assert_eq!(result, "281".to_string());
    }
}
