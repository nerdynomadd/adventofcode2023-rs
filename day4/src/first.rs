pub struct Scratchcard {
    winning_numbers: Vec<i32>,
    scratch_numbers: Vec<i32>
}

pub fn main(file_content: &String) -> u32 {
    let scratchcards = parse_input_scratchcards(file_content);

    scratchcards.iter().map(|card| calculate_points(card)).sum()
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

fn calculate_points(card: &Scratchcard) -> u32 {
    /**
    The first match makes the card worth one point and each match after the first doubles the point value of that card.
    For example, if a card has 4 matches, it is worth 8 points.
    */
    let mut points = 0;

    for scratch_number in &card.scratch_numbers {
        if card.winning_numbers.contains(scratch_number) {
            points += 1;
        }
    }

    if points > 0 {
        points = 2_u32.pow(points - 1);
    }

    points
}