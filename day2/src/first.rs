pub fn main(file_content: &String) -> u32 {
    let mut lines = file_content.lines();

    let mut sum = 0;

    for (i, line) in lines.enumerate() {
        let line = line.trim_start_matches(format!("Game {}: ", i + 1).as_str());

        let mut validation_count = 0;

        for game_line in line.split("; ") {
            let mut set_of_cube = SetOfCube::default();

            for i in game_line.split(", ") {
                let mut split = i.split(" ");

                let cube_number = split.next().unwrap().parse::<u32>().unwrap();
                let cube_color = split.next().unwrap();

                match cube_color {
                    "red" => set_of_cube.red += cube_number,
                    "green" => set_of_cube.green += cube_number,
                    "blue" => set_of_cube.blue += cube_number,
                    _ => panic!("Unknown color: {}", cube_color),
                }
            }

            if set_of_cube.is_valid() {
                validation_count += 1;
            }
        }

        if validation_count == line.split("; ").count() {
            sum += i as u32 + 1;
        }
    }

    sum as u32
}

#[derive(Default)]
struct SetOfCube {
    index: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl SetOfCube {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}