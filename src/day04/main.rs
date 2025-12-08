use std::fs;

fn main() -> std::io::Result<()> {

    let input= fs::read_to_string("src/day04/input.txt");
    let mut lines = Vec::new();

    let mut map:Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
    let mut acessibles = 0;

    for line in input.unwrap().lines() {
        lines.push(line.to_string())
    }

    for line_index in 0..lines.len() {

        let line = &lines[line_index];

        let line_bytes = line.as_bytes();

        map.push(Vec::new());
        map[line_index].push(0);

        for i in 0..line_bytes.len() {
            if line_bytes[i] == b'.' {
                map[line_index].push(0)
            } else if line_bytes[i] == b'@' {
                map[line_index].push(1)
            }
        }
        
        map[line_index].push(0);
    }
    
    map = create_padding(map, lines[0].len());

    loop {
        let mut delta = 0;
        for r in 1..map.len() - 1 {
            for c in 1..map[0].len() - 1 {
                if map[r][c] != 1 { continue }
                let adjacent = look_around(r, c, &map);
                if adjacent < 4 {
                    acessibles += 1;
                    map[r][c] = 0;
                    delta += 1;
                }
            }
        }
        if delta == 0 { break }
    }

    println!("{:?}", acessibles);

    Ok(())
}

fn create_padding(mut mapi: Vec<Vec<u8>>, row_len: usize) -> Vec<Vec<u8>> {
    mapi.insert(0, vec![0; row_len+2]);
    mapi.push(vec![0; row_len+2]);
    return mapi
}

fn look_around(row:usize, column:usize, map: &Vec<Vec<u8>>) -> u8 {
    let mut neighbors = 0u8;
    
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 { continue }
            if map[row + i - 1][column + j - 1] == 1 { neighbors += 1 }
        }
    }
    
    return neighbors
}