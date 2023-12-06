use std::collections::HashMap;

pub fn main(file_content: &String) -> u32 {
    let lines = file_content.lines();

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    let mut sum = 0;

    for (_, line) in lines.enumerate() {
        let mut line_numbers: Vec<Number> = Vec::new();
        let mut line_symbols = Vec::new();

        let mut chars = line.chars();
        let mut j = 0;
        let mut line_number_index = 0;
        let mut number_started = false;

        while let Some(character) = chars.next() {
            // If the character is a number, and we are already tracking a number, we need to complement it
            // Otherwise, if we are no longing tracking a number and it is a ".", we need to push a new number only once
            // If it is a symbol, we need to push a new symbol
            if character.is_digit(10) {
                number_started = true;
                if let Some(number) = line_numbers.get_mut(line_number_index) {
                    number.value = number.value * 10 + character.to_digit(10).unwrap();
                    number.ending_position = j;
                } else {
                    line_numbers.push(Number {
                        value: character.to_digit(10).unwrap(),
                        starting_position: j,
                        ending_position: j,
                    });
                }
            } else if character == '.' && number_started {
                line_number_index += 1;
                number_started = false;
            } else if character != '.' {
                line_symbols.push(Symbol {
                    position: j,
                    value: character,
                });
            }

            j += 1;
        }

        numbers.push(line_numbers);
        symbols.push(line_symbols);
    }

    // We need to find adjecent numbers to symbols in the line above and below
    // Make sure to check for special cases:
    // 1. We need to check the line above and below and to the left and right and the diagonals of the symbol itself
    // 2. A number can match the adjacent by being around with it's starting and ending position
    for (i, line_symbols) in symbols.iter().enumerate() {
        for symbol in line_symbols {
            let mut adjacent_numbers = Vec::new();

            let mut check_line = move |number: &Number, symbol_position: usize| (number.starting_position >= symbol.position - 1 && number.starting_position <= symbol.position + 1) || (number.ending_position >= symbol.position - 1 && number.ending_position <= symbol.position + 1) ;

            // Check the line above
            if numbers[i - 1].len() > 0 {
                for number in &numbers[i - 1] {
                    if check_line(number, symbol.position) {
                        adjacent_numbers.push(number.value);
                    }
                }
            }

            // Check the line below
            if numbers[i + 1].len() > 0 {
                for number in &numbers[i + 1] {
                    if check_line(number, symbol.position) {
                        adjacent_numbers.push(number.value);
                    }
                }
            }

            // Check the line to the left and the right
            for number in &numbers[i] {
                if number.starting_position == symbol.position - 1 || number.ending_position == symbol.position - 1 || number.starting_position == symbol.position + 1 || number.ending_position == symbol.position + 1 {
                    adjacent_numbers.push(number.value);
                }
            }

            sum += adjacent_numbers.iter().sum::<u32>();
        }
    }

    sum
}

pub struct Number {
    pub value: u32,
    pub starting_position: usize,
    pub ending_position: usize,
}

pub struct Symbol {
    pub position: usize,
    pub value: char,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let file_content = String::from(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
        );

        assert_eq!(main(&file_content), 4361);
    }
}