fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input:&str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(

"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
");
        assert_eq!(result, "142");
    }
}

