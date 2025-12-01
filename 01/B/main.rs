use std::fs;

fn main() {
    let data = fs::read_to_string("../data").expect("Unable to read file");
    let mut result = 0;
    let mut dial_counter = 50;

    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next().unwrap();
        let step: i32 = line[1..].parse().unwrap();

        let passes_through_zero = step / 100;
        result += passes_through_zero;

        let remaining_step = step % 100;
        let new_position = if direction == 'L' {
            let pos = (dial_counter - remaining_step).rem_euclid(100);
            if pos > dial_counter {
                result += 1;
            }
            pos
        } else {
            let pos = (dial_counter + remaining_step) % 100;
            if pos < dial_counter {
                result += 1;
            }
            pos
        };

        dial_counter = new_position;
    }

    println!("{}", result);
}
