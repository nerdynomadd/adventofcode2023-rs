use std::iter::Iterator;

mod first;
mod second;

const ORDER: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

fn main() {
    let file_content = std::fs::read_to_string("assets/seventh.txt").unwrap();

    let first_result = first::main(&file_content);
    let second_result = second::main(&file_content);

    println!("First result: {}", first_result);
    println!("Second result: {}", second_result);
}
