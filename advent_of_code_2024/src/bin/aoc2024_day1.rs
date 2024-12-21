use std::{collections::HashMap, io};

use advent_of_code_2024::read_lines;


fn main() -> io::Result<()> {
    // extract columns of numbers
    let mut first_column = Vec::new();
    let mut second_column = Vec::new();
    
    for line in read_lines("src/bin/aoc2024_day1/input")? {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace();
        let first = tokens.next().unwrap().parse::<u32>().unwrap();
        let second = tokens.next().unwrap().parse::<u32>().unwrap();

        first_column.push(first);
        second_column.push(second);
    }

    // sort
    first_column.sort();
    second_column.sort();

    // tally differences and print result
    let mut total_difference = 0;
    for (&first, &second) in first_column.iter().zip(second_column.iter()) {
        total_difference += u32::max(first, second) - u32::min(first, second);
    }

    println!("total difference: {total_difference}");

    // count occurrences
    let first_counts = count_occurrences(&first_column);
    let second_counts = count_occurrences(&second_column);

    // tally similarity and print result
    let mut total_similarity = 0;
    for (item, first_count) in first_counts {
        if let Some(second_count) = second_counts.get(&item) {
            total_similarity += item * first_count * second_count;
        }
    }

    println!("total similarity: {total_similarity}");
    
    Ok(())
}

fn count_occurrences(list: &Vec<u32>) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    for &item in list {
        match map.get(&item) {
            Some(&count) => map.insert(item, count + 1),
            None => map.insert(item, 1),
        };
    }

    map
}