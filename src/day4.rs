use std::env;
use aoc::read_file_lines;


fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &str = &args[1];
    let lines = read_file_lines(path);

    println!("Sum is {}", get_sum(&lines));
    println!("Number of processed games is {}", expand_cards(&lines));
}

fn count_winning_numbers(line: &String) -> usize {
    let (win_str, nums_str) = line.split_once(':').unwrap().1.split_once('|').unwrap();
    let win = win_str.split_whitespace().collect::<Vec<&str>>();
    let nums = nums_str.split_whitespace();

    let len = nums.filter(|num| win.contains(num)).collect::<Vec<_>>().len();
    return len;
}

fn expand_cards(lines: &Vec<String>) -> usize {
    
    let mut counts = vec![1; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let wins = count_winning_numbers(line);

        let mut j = 1;
        while j <= wins {
            counts[i + j] += counts[i];
            j += 1;
        }
    }

    counts.iter().sum()
}

fn get_sum(lines: &Vec<String>) -> usize {
    let mut sum = 0;


    for line in lines.iter() {
        let len = count_winning_numbers(line);
        sum += (1 << len) / 2;
    }
    return sum;
}
