use std::fs;

fn main() {
    let mut password = 0;
    let mut current = 50;
    let input = fs::read_to_string("src/day01/input.txt");
    for line in input.unwrap().lines() {
        //false = counter-clockwise
        let mut direction = true;
        let line_str = line.to_string();
        if line_str.as_bytes()[0] == b'L' {
            direction = false;
        }
        let mut magnitude = (&line_str[1..]).parse::<u32>().unwrap();
        while magnitude > 0 {
            if direction {
                if current == 99 {
                    current = 0
                } else {
                    current += 1
                }
            } else {
                if current == 0 {
                    current = 99
                } else {
                    current -= 1
                }
            }
            magnitude -= 1;
            if current == 0 {
                password += 1
            }
        }
    }
    println!("{}", password)
}