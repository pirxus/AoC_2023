use std::fmt::Display;
use std::env;
use aho_corasick::AhoCorasick;
use aoc::read_file_lines;

const PRINT: bool = false;
const PART: u8 = 2;



fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &str = &args[1];
    let lines = read_file_lines(path);
    if PRINT {
        print_vec_lines(&lines);
    }

    let sum: u32 = match PART {
        1 => get_calibration_sum(&lines),
        2 => get_calibration_sum_corrected(&lines),
        _ => 0,
    };

    println!("Calibration sum is {}", sum)
}


fn get_calibration_sum_corrected(lines: &Vec<String>) -> u32 {

    let numbers = "one 1 two 2 three 3 four 4 five 5 six 6 seven 7 eight 8 nine 9";
    let patterns: Vec<&str> = numbers.split(' ').collect();

    let mut sum: u32 = 0;

    let ac = AhoCorasick::new(patterns).unwrap();

    for line in lines.iter() {

        let result = ac.find_overlapping_iter(line).collect::<Vec<_>>();
        let first = result.iter().nth(0).unwrap().pattern().as_u32();
        let last = result.iter().last().unwrap().pattern().as_u32();

        sum += 10 * ((first / 2) + 1) + (last / 2 + 1);
    }

    return sum;
}

fn get_calibration_sum(lines: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in lines.iter() {

        for c in line.chars() {
            if c.is_numeric() {
                sum += 10 * c.to_digit(10).unwrap();
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_numeric() {
                sum += c.to_digit(10).unwrap();
                break;
            }
        }
    }

    return sum;
}

fn print_vec_lines<T>(vector: &Vec<T>) 
    where
        T: Display,
{
    for line in vector.iter() {
        println!("{}", line);
    }
}
