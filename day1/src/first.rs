pub fn main(file_content: &String) -> u32 {
    let file_content = file_content.lines();

    let mut sum: u32 = 0;

    file_content
        .for_each(|line| {
            let first_pos = line.find(char::is_numeric).unwrap();
            let first = line.chars().nth(first_pos).unwrap();
            let second_pos = line.rfind(char::is_numeric).unwrap();
            let second = line.chars().nth(second_pos).unwrap();

            // Could be done with first * 10 + second if we were to parse them earlier
            sum += format!("{}{}", first, second).parse::<u32>().unwrap();
        });

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_digits() {
        let line = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".to_string();

        assert_eq!(super::main(&line), 142);
    }
}