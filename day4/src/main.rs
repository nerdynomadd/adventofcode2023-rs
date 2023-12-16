mod first;
mod second;

fn main() {
    let file_content = std::fs::read_to_string("assets/fourth.txt").unwrap();

    let first_result = first::main(&file_content);
    let second_result = second::main(&file_content);

    println!("First result: {}", first_result);
    println!("Second result: {}", second_result);
}
