pub fn main(file_content: &String) -> u32 {
    let mut lines = file_content.lines();

    let mut total = 1;

    let mut times: Vec<usize> = Vec::new();
    let mut distances: Vec<usize> = Vec::new();

    // Parse the following two lines and put into respective vectors
    // Time:      7  15   30
    // Distance:  9  40  200
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
            // Don't hold the button at all (that is, hold it for 0 milliseconds) at the start of the race. The boat won't move; it will have traveled 0 millimeters by the end of the race.
            // Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at a speed of 1 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of 6 millimeters.
            // Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond. It will then get 5 milliseconds to move, reaching a total distance of 10 millimeters.
            // Hold the button for 3 milliseconds. After its remaining 4 milliseconds of travel time, the boat will have gone 12 millimeters.
            // Hold the button for 4 milliseconds. After its remaining 3 milliseconds of travel time, the boat will have gone 12 millimeters.
            // Hold the button for 5 milliseconds, causing the boat to travel a total of 10 millimeters.
            // Hold the button for 6 milliseconds, causing the boat to travel a total of 6 millimeters.
            // Hold the button for 7 milliseconds. That's the entire duration of the race. You never let go of the button. The boat can't move until you let go of the button. Please make sure you let go of the button so the boat gets to move. 0 millimeters.
x
            let traveled_distance = i * (time - i);

            if traveled_distance > *distance {
                number_best_times += 1;
            }
        }

        total *= number_best_times;
    }

    total as u32
}