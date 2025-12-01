use std::fs;

fn main() {
    let data = fs::read_to_string("../data")
        .expect("Failed to read file");
    
    let mut dial_counter = 50;
    let mut result = 0;
    
    for line in data.lines() {
        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();
        
        if direction == 'L' {
            dial_counter -= distance;
        } else {
            dial_counter += distance;
        }
        
        dial_counter = dial_counter.rem_euclid(100);
        
        println!("{} -> {}", line.trim(), dial_counter);
        
        if dial_counter == 0 {
            result += 1;
        }
    }
    
    println!("\nPassword: {}", result);
}
