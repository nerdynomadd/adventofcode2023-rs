mod first;

fn main() {
    let file_content = std::fs::read_to_string("assets/third.txt").unwrap();

    let first_result = first::main(&file_content);

    println!("First result: {}", first_result);
}
