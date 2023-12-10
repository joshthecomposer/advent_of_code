fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let groups: Vec<&str> = input.split("\n\n").collect();
    let mut sums: Vec<i32> = Vec::new();

    for group in groups {
        let sum = group.split("\n")
            .filter_map(|s| s.parse::<i32>().ok())
            .sum();
        sums.push(sum);
    }

    sums.sort_unstable_by(|a, b| b.cmp(a));
    sums.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
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
        assert_eq!(45000, result);
    }
}
