pub fn main(file_content: &String) -> u32 {
    let mut lines = file_content.lines();

    let mut games = Vec::new();

    for (i, line) in lines.enumerate() {
        let line = line.trim_start_matches(format!("Game {}: ", i + 1).as_str());

        let mut set_of_cubes = Vec::new();

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

            set_of_cubes.push(set_of_cube);
        }

        games.push(set_of_cubes);
    }

    games.iter().map(|games| {
        let mut max_set_of_cube = SetOfCube::default();
        for set_of_cube in games {
            max_set_of_cube = max_set_of_cube.max(set_of_cube);
        }
        max_set_of_cube.red * max_set_of_cube.green * max_set_of_cube.blue
    }).sum::<u32>().into()
}

#[derive(Default)]
struct SetOfCube {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl SetOfCube {
    fn max(&self, other: &SetOfCube) -> SetOfCube {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}