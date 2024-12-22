use std::io;

use advent_of_code_2024::read_lines;

fn main() -> io::Result<()> {
    let lines = read_lines("src/bin/input/aoc2024_day4")?
        .map(|x|{x.unwrap()}) // unwrap each line
        .map(|s|{s.chars().collect::<Vec<_>>()}) // convert strings to char vecs
        .collect::<Vec<_>>(); // collect iterator over lines into vec

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            for direction in directions {
                if check_word_in_direction(&lines, "XMAS", (i as isize, j as isize), direction) {
                    count += 1;
                }
            }
        }
    }

    let mut count_mas_cross = 0;
    for i in 1..lines.len() - 1 {
        for j in 1..lines[0].len() - 1 {
            count_mas_cross += check_for_mas_crosses(&lines, (i as isize, j as isize));
        }
    }

    println!("count xmas: {count}");
    println!("count mas cross: {count_mas_cross}");
    
    Ok(())
}

fn check_word_in_direction(lines: &Vec<Vec<char>>, word: &str, coord: (isize, isize), direction: (isize, isize)) -> bool {
    let mut current_coord = coord;
    
    for char in word.chars() {
        if current_coord.0 < 0 || current_coord.0 >= lines.len() as isize {
            return false;
        }
        if current_coord.1 < 0 || current_coord.1 >= lines[0].len() as isize {
            return false;
        }
        if lines[current_coord.0 as usize][current_coord.1 as usize] != char {
            return false;
        }

        current_coord = (current_coord.0 + direction.0, current_coord.1 + direction.1);
    }

    return true;
}

fn check_for_mas_crosses(lines: &Vec<Vec<char>>, coord: (isize, isize)) -> u32 {
    let mut count = 0;

    if lines[coord.0 as usize][coord.1 as usize] != 'A' {
        return count;
    }

    // looks like this wasn't supposed to be included!
    // if
    //     check_ms(&lines, coord, (0, 1)) &&
    //     check_ms(&lines, coord, (1, 0)) {
    //         count += 1;
    // }

    if
        check_ms(&lines, coord, (1, 1)) &&
        check_ms(&lines, coord, (-1, 1)) {
            count += 1;
    }

    count
}

fn check_ms(lines: &Vec<Vec<char>>, center: (isize, isize), direction: (isize, isize)) -> bool {
    let first = (center.0 + direction.0, center.1 + direction.1);
    let second = (center.0 - direction.0, center.1 - direction.1);

    let first = lines[first.0 as usize][first.1 as usize];
    let second = lines[second.0 as usize][second.1 as usize];
    
    match (first, second) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => false,
    }
}