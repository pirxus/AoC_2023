use std::env;
use aoc::read_file_lines;
use std::cmp::{min, max};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &str = &args[1];
    let lines = read_file_lines(path);

    println!("Sum is {}", get_sum(&lines));
    println!("Ratio is {}", get_ratio(&lines));
}

fn get_ratio(lines: &Vec<String>) -> usize {
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let mut j = 0;

        while j < line.len() {

            let c = line.chars().nth(j).unwrap();
            if c.is_digit(10) {

                // measure the length of the number
                let mut len = 1;
                while j + len < line.len()
                    && line.chars().nth(j + len).unwrap().is_digit(10) {
                        len += 1;
                }

                // scan around the number
                
                let x_min: usize = max(0, j as i32 - 1).try_into().unwrap();
                let x_max: usize = min(line.len() - 1, j + len);
                let y_min: usize = max(0, i as i32 - 1).try_into().unwrap();
                let y_max: usize = min(lines.len() - 1, i + 1);

                let mut k = y_min;
                while k <= y_max {

                    let mut l = x_min;
                    while l <= x_max {
                        if lines[k].chars().nth(l).unwrap() == '*' {
                            let num_str: &str = &line[j..j+len];
                            let num = num_str.parse::<usize>().unwrap();

                            if gears.contains_key(&(k, l)) {
                                if let Some(entry) = gears.get_mut(&(k, l)) {
                                    entry.push(num);
                                }
                            } else {
                                gears.insert((k, l), vec![num]);
                            }
                        }

                        l += 1;
                    }
                    k += 1;
                }


                // jump over the rest of the number
                j += len - 1;
            }

            j += 1;
        }
    }

    gears.iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().product())
        .collect::<Vec<usize>>()
        .iter()
        .sum()
}

fn get_sum(lines: &Vec<String>) -> usize {
    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        let mut j = 0;

        while j < line.len() {

            let c = line.chars().nth(j).unwrap();
            if c.is_digit(10) {

                // measure the length of the number
                let mut len = 1;
                while j + len < line.len()
                    && line.chars().nth(j + len).unwrap().is_digit(10) {
                        len += 1;
                }

                let mut adj = false;
                // scan around the number
                
                let x_min: usize = max(0, j as i32 - 1).try_into().unwrap();
                let x_max: usize = min(line.len() - 1, j + len);
                let y_min: usize = max(0, i as i32 - 1).try_into().unwrap();
                let y_max: usize = min(lines.len() - 1, i + 1);

                let mut k = y_min;
                while k <= y_max {

                    let mut l = x_min;
                    while l <= x_max {
                        if is_symbol(lines[k].chars().nth(l).unwrap()) {
                            adj = true;
                        }

                        l += 1;
                    }
                    k += 1;
                }


                // finally, read in the number if adjacent to a symbol
                if adj {
                    let num: &str = &line[j..j+len];
                    sum += num.parse::<usize>().unwrap();
                }

                // jump over the rest of the number
                j += len - 1;
            }

            j += 1;
        }
    }
    return sum;
}

fn is_symbol(c: char) -> bool {
    if c.is_ascii() && c != '.' && !c.is_digit(10) {
        return true;
    } else {
        return false;
    }
}
