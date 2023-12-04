pub fn main(file_content: &String) -> u32 {
    let file_content = file_content.lines();

    let mut sum: u32 = 0;

    file_content
        .for_each(|line| {
            let (first, second) = find_digits(line);

            sum += first * 10 + second;
        });

    sum
}

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_digits(line: &str) -> (u32, u32) {
    let mut first = None;
    let mut second = 0;

    let chars = line.as_bytes();
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            first = first.or(c.to_digit(10));
            second = c.to_digit(10).unwrap();
        } else {
            for (j, d) in DIGITS.iter().enumerate() {
                if chars[i..].starts_with(d.as_bytes()) {
                    first = first.or(Some(j as u32 + 1));
                    second = j as u32 + 1;
                }
            }
        }
    }

    (first.unwrap(), second)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_digits() {
        let line = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen".to_string();

        assert_eq!(super::main(&line), 281);
    }
}