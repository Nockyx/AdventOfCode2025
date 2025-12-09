use std::fs;

fn main() -> std::io::Result<()> {

    part2();

    Ok(())
}

#[allow(dead_code)]
fn part1() {

    let input= fs::read_to_string("src/day05/input.txt");
    let mut lines = Vec::new();

    for line in input.unwrap().lines() {
        lines.push(line.to_string())
    }

    let mut ranges: Vec<(u128, u128)> = Vec::new();
    let mut ids: Vec<u128> = Vec::new();
    let mut fresh = 0u128;
    let mut fresh_ids: Vec<u128> = Vec::new();

    for line in lines {
        if line.contains("-") {
            let (x, y) = line.split_once("-").expect("hyphen");
            ranges.push((x.parse::<u128>().unwrap(), y.parse::<u128>().unwrap()));
        } else if line != "" {
            ids.push(line.parse::<u128>().unwrap())
        }
    }

    for id in &ids {
        for range in &ranges {
            if *id > range.0 && *id < range.1 && !fresh_ids.contains(id) { fresh += 1; fresh_ids.push(*id) }
        }
    }

    //println!("{:?}", ranges);
    //println!("{:?}", ids);
    println!("{:?}", fresh);

}

fn part2() {

    let input= fs::read_to_string("src/day05/test.txt");
    let mut lines = Vec::new();

    for line in input.unwrap().lines() {
        lines.push(line.to_string())
    }

    let mut ranges: Vec<(u128, u128)> = Vec::new();
    let mut ids= 0u128;
    let mut new_ranges: Vec<(u128, u128)> = Vec::new();

    for line in lines {
        if line.contains("-") {
            let (x, y) = line.split_once("-").expect("hyphen");
            ranges.push((x.parse::<u128>().unwrap(), y.parse::<u128>().unwrap()));
        }
    }

    // cases
    
    // case 1 = ---------
    //                ----------

    // case 2 = ---------
    //      ----------

    // case 3 = ---------
    //            ----

    // case 4 = ---------
    //        -------------
    
    for range_id in 0..ranges.len() {
        let (mut r_start, mut r_end) = ranges[range_id];
        //println!("{:?}", (r_start, r_end));
        for (c_start, c_end) in &mut ranges[range_id+1..] {
            
            //case 3
            if *c_start >= r_start && *c_end <= r_end {
                *c_start = 0;
                *c_end = 0;
                continue
            }
            //case 1
            if *c_start <= r_end && *c_start >= r_start {
                r_end = *c_start - 1
            }
            //case 2
            if *c_end >= r_start && *c_end <= r_end {
                r_start = *c_end + 1
            }
            //case 4
            if *c_start <= r_start && *c_end >= r_end {
                r_start = 0;
                r_end = 0;
            }
        }
        new_ranges.push((r_start,r_end))
    }
    
    //println!("{:?}", ranges);
    //println!("{:?}", new_ranges);
    
    for i in new_ranges {
        if i == (0, 0) { continue }
        ids += (i.1-i.0) + 1
    }
    
    println!("{}", ids)

}