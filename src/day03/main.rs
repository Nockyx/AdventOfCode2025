use std::fs;

fn main() {
    part2()
}

#[allow(dead_code)]
fn part1() {

    let input= fs::read_to_string("src/day03/input.txt");
    let mut lines = Vec::new();
    let mut jolts = 0i128;

    for line in input.unwrap().lines() {
        lines.push(line.to_string())
    }

    for bank in lines {

        let bank_bytes = bank.as_bytes();
        let mut current_choice = 0i128;

        for i in 0..bank_bytes.len() {
            let battery = bank_bytes[i];
            for second_battery in &bank_bytes[i+1..] {
                let pair = format!("{}{}", battery as char, *second_battery as char).parse::<i128>().unwrap();
                if pair > current_choice { current_choice = pair}
            }
        }
        println!("{}", current_choice);
        jolts += current_choice

    }

    println!("{:?}", jolts)

}

fn part2() {

    let input= fs::read_to_string("src/day03/input.txt");
    let mut lines = Vec::new();
    let mut jolts = 0i128;

    for line in input.unwrap().lines() {
        lines.push(line.to_string())
    }

    for line in lines {
        let mut num_idxs: Vec<usize>= Vec::new();
        let nums: Vec<u32> = line.as_bytes().iter().map(|x| (*x as char).to_string().parse::<u32>().unwrap()).collect();

        while num_idxs.len() < 12 {
            let mut high = (0, 0);
            for battery_index in last_index(&num_idxs) as i8..=(line.len() as i8 - (12 - num_idxs.len() as i8)) {
                let num= nums[battery_index as usize];
                if num == 9 {
                    high.1 = battery_index;
                    break
                } else {
                    if num > high.0 {
                        high.0 = num;
                        high.1 = battery_index
                    }
                }
            }
            num_idxs.push(high.1 as usize)
        }

        let mut number = String::new();

        for index in num_idxs.clone() {
            number = format!("{}{}", number, nums[index])
        }
        println!("{}", number);
        jolts += number.parse::<i128>().unwrap()

    }

    println!("{}", jolts)

}

fn last_index(list: &Vec<usize>) -> usize {
    let a = list.last();
    match a {
        Some(&value) => value + 1,
        None => 0usize,
    }
}