pub fn main(file_content: &String) -> u32 {
    let histories = parse_input(file_content);

    sum_extrapolated_values(histories) as u32
}

pub fn parse_input(input: &String) -> Vec<Vec<i32>> {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|number| {
                number.parse::<i32>().unwrap()
            })
            .collect()
    }).collect()
}

fn generate_diff_sequences(history: &[i32]) -> Vec<Vec<i32>> {
    let mut sequences = vec![history.to_vec()];
    loop {
        let last_sequence = sequences.last().unwrap();
        let mut new_sequence = vec![];

        for i in 0..last_sequence.len() - 1 {
            new_sequence.push(last_sequence[i + 1] - last_sequence[i]);
        }

        if new_sequence.iter().all(|&x| x == 0) {
            break;
        }

        sequences.push(new_sequence);
    }

    sequences
}

fn extrapolate_next_value(sequences: &[Vec<i32>]) -> i32 {
    let mut next_value = *sequences[0].last().unwrap();
    for i in (1..sequences.len()).rev() {
        next_value += sequences[i].last().unwrap();
    }
    next_value
}

fn sum_extrapolated_values(histories: Vec<Vec<i32>>) -> i32 {
    histories.into_iter().map(|history| {
        let sequences = generate_diff_sequences(&history);
        extrapolate_next_value(&sequences)
    }).sum()
}