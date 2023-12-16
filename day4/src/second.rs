#[derive(Debug, PartialEq)]
pub struct Scratchcard {
    winning_numbers: Vec<i32>,
    scratch_numbers: Vec<i32>
}

impl Eq for Scratchcard {

}

pub fn main(file_content: &String) -> u32 {
    let scratchcards = parse_input_scratchcards(file_content);

    process_scratchcards(&scratchcards) as u32
}

fn parse_input_scratchcards(file_content: &String) -> Vec<Scratchcard> {
    file_content.lines().map(|line| {
        let part: Vec<&str> = line.split(": ").collect();
        let part: Vec<&str> = part[1].split("|").collect();
        let winning_numbers: Vec<i32> = part[0].split_whitespace().map(|n| n.parse().unwrap()).collect();
        let scratch_numbers: Vec<i32> = part[1].split_whitespace().map(|n| n.parse().unwrap()).collect();
        Scratchcard { winning_numbers, scratch_numbers }
    }).collect()
}

fn process_scratchcards(cards: &Vec<Scratchcard>) -> usize {
    // Create a vector to store the total cards for each card by creating it from the length of the cards vector, and initializing it with 0
    let mut total_cards_vector = vec![1; cards.len()];

    for card in cards {
        let mut current_card_index = cards.iter().position(|c| c == card).unwrap();
        println!("Current card index: {}", current_card_index);
        // Multiplier is the number of times we have a card and we should unwind it, if it's 0, we should unwind it once
        let multiplier = total_cards_vector[current_card_index];
        for _ in 0..multiplier {
            let mut current_card_index = cards.iter().position(|c| c == card).unwrap();
            for scratch_number in &card.scratch_numbers {
                if card.winning_numbers.contains(scratch_number) {
                    current_card_index += 1;
                    total_cards_vector[current_card_index] += 1;
                }
            }
        }
    }

    total_cards_vector.iter().sum()
}