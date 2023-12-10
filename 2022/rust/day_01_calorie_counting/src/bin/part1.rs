fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let groups: Vec<&str> = input.split("\n\n").collect();
    let mut max = 0;
    for group in groups {
        let sum = group.split("\n")
            .filter_map(|n| n.parse::<i32>().ok())
            .sum();
        if sum > max {
            max = sum;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
",
        );
        assert_eq!(24000, result);
    }
}
