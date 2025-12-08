use std::fs;

fn main() {

    let input = fs::read_to_string("src/day02/input.txt").unwrap().to_string();

    let ranges = input.split(",").into_iter().map(|x| {x.split("-").into_iter().map(|x| { x.parse::<u128>().unwrap() }).collect::<Vec<_>>()}).collect::<Vec<_>>();

    let mut sum = 0;

    let mut repeated : Vec<u128> = Vec::new();

    for i in 1..=100000 {
        let n = 12 / i.to_string().len();

        for j in 2..=n {
            let num = recursive_concat(j, i).parse::<u128>().unwrap();
            for range in &ranges {
                if num >= range[0] && num <= range[1] && !repeated.contains(&num) {
                    println!("{}", num);
                    repeated.push(num);
                    sum += num;
                }
            }
        }

    }

    println!("{}", sum)

}

fn recursive_concat(mut repeats:usize, number:u32) -> String {
    if repeats <= 0 { return String::from("")}
    repeats -= 1;
    let num = format!("{}{}", number, recursive_concat(repeats, number));
    return num
}