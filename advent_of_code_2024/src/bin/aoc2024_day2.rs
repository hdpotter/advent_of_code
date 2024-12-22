use std::io;

use advent_of_code_2024::read_lines;

fn main() -> io::Result<()> {
    let reports = read_lines("src/bin/input/aoc2024_day2")?;

    let mut safe_count = 0;
    let mut safe_with_dampener_count = 0;

    for report in reports {
        let report = report?;
        let data = report.split_whitespace();

        let mut safe = safe_without_dampener(data.clone(), None);

        if safe {
            safe_count += 1;
        }

        for i in 0..data.clone().count() {
            let skip_safe = safe_without_dampener(data.clone(), Some(i));

            safe = safe || skip_safe;
        }

        if safe {
            safe_with_dampener_count += 1;
        }
    }

    println!("safe count: {safe_count}");
    println!("safe count with dampener: {safe_with_dampener_count}");

    Ok(())
}

fn safe_without_dampener<'a, T: Iterator<Item = &'a str>>(mut data: T, skip: Option<usize>) -> bool {
    let mut index = 0;
    
    if skip == Some(0) {
        data.next();
        index += 1;
    }

    let mut current_level = data.next().unwrap().parse::<i32>().unwrap(); // in given input, all reports are non-empty
    let mut increasing = None;

    let mut safe = true;

    index += 1; // set index to refer to new, not current

    while let Some(new_level) = data.next() {
        if skip == Some(index) {
            index += 1;
            continue;
        }

        let new_level = new_level.parse::<i32>().unwrap();
        match new_level - current_level {
            -1 | -2 | -3 => match increasing {
                Some(true) => {
                    safe = false;
                    break;
                }, // change in direction
                Some(false) => (), // good
                None => increasing = Some(false), // good, detected decreasing
            },
            1 | 2 | 3 => match increasing {
                Some(true) => (), // good
                Some(false) => {
                    safe = false;
                    break;
                }, // change in direction
                None => increasing = Some(true), // good, detected increasing
            },
            _ => {
                safe = false;
                break;
            },
        }

        current_level = new_level;
        index += 1;
    }

    safe
}