pub fn main(file_content: &String) -> u32 {
    let mut lines = file_content.lines();

    let mut total = 1;

    let mut times: Vec<usize> = Vec::new();
    let mut distances: Vec<usize> = Vec::new();

    let mut split_time = lines.next().unwrap().split_whitespace().skip(1);
    let mut split_distance = lines.next().unwrap().split_whitespace().skip(1);


    while let Some(time) = split_time.next() {
        let time = time.parse::<usize>().unwrap();

        times.push(time);
    }

    while let Some(distance) = split_distance.next() {
        let distance = distance.parse::<usize>().unwrap();

        distances.push(distance);
    }

    let combination = times.iter().zip(distances.iter());

    for (time, distance) in combination {
        let mut number_best_times = 0;
        for i in 0..*time {
            let traveled_distance = i * (time - i);

            if traveled_distance > *distance {
                number_best_times += 1;
            }
        }

        total *= number_best_times;
    }

    total as u32
}