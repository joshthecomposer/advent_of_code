use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = find_solution(input);
    let duration = start.elapsed();
    dbg!(output);
    println!("Executed in: {:?}", duration);
}

fn find_solution(input: &str) -> u32 {
    let mut count = 0;
    let word_len = 4;

    let matrix: Vec<Vec<&str>> = input
        .lines()
        .map(|r| r.split("").filter(|c| !c.is_empty()).collect())
        .collect();

    let width = matrix[0].len();
    let height = matrix.len();

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &"X" {
                // Check DOWN
                if i + word_len - 1 < height
                    && matrix[i + 1][j] == "M"
                    && matrix[i + 2][j] == "A"
                    && matrix[i + 3][j] == "S"
                {
                    count += 1;
                }

                // Check RIGHT
                if j + word_len - 1 < width
                    && matrix[i][j + 1] == "M"
                    && matrix[i][j + 2] == "A"
                    && matrix[i][j + 3] == "S"
                {
                    count += 1;
                }

                // Check UP
                if i >= word_len - 1
                    && matrix[i - 1][j] == "M"
                    && matrix[i - 2][j] == "A"
                    && matrix[i - 3][j] == "S"
                {
                    count += 1;
                }

                // Check LEFT
                if j >= word_len - 1
                    && matrix[i][j - 1] == "M"
                    && matrix[i][j - 2] == "A"
                    && matrix[i][j - 3] == "S"
                {
                    count += 1;
                }

                // Check RIGHT UP
                if j + word_len - 1 < width
                    && i >= word_len - 1
                    && matrix[i - 1][j + 1] == "M"
                    && matrix[i - 2][j + 2] == "A"
                    && matrix[i - 3][j + 3] == "S"
                {
                    count += 1;
                }

                // Check LEFT UP
                if j >= word_len - 1
                    && i >= word_len - 1
                    && matrix[i - 1][j - 1] == "M"
                    && matrix[i - 2][j - 2] == "A"
                    && matrix[i - 3][j - 3] == "S"
                {
                    count += 1;
                }

                // Check LEFT DOWN
                if j >= word_len - 1
                    && i + word_len - 1 < height
                    && matrix[i + 1][j - 1] == "M"
                    && matrix[i + 2][j - 2] == "A"
                    && matrix[i + 3][j - 3] == "S"
                {
                    count += 1;
                }

                // Check RIGHT DOWN
                if j + word_len - 1 < width
                    && i + word_len - 1 < height
                    && matrix[i + 1][j + 1] == "M"
                    && matrix[i + 2][j + 2] == "A"
                    && matrix[i + 3][j + 3] == "S"
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = find_solution(
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
);
        assert_eq!(18, result);
    }
}
