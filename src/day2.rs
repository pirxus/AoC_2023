use std::env;
use aoc::read_file_lines;

const PART: i32 = 2;

fn get_sum(lines: Vec<String>) -> usize {
    let limit = vec![12, 13, 14];

    let mut sum: usize = 0;
    for (i, game) in lines.iter().enumerate() {
        let game_res: &str = game.split(':').collect::<Vec<_>>()[1].trim();

        let draws: Vec<&str> = game_res.split(';').map(|a| a.trim()).collect();

        let vals: Vec<Vec<&str>> = draws.iter()
            .map(|a| a.split(',')
                 .map(|b| b.trim())
                 .collect())
            .collect();
        
        let mut possible: bool = true;
        for draw in vals.iter() {
            for val in draw.iter() {

                let [val, col] = val.split(' ').collect::<Vec<_>>().try_into().unwrap();
                match col {
                    "red" => {
                        if val.parse::<usize>().unwrap() > limit[0] {
                            possible = false;
                            break;
                        }
                    },
                    "green" => {
                        if val.parse::<usize>().unwrap() > limit[1] {
                            possible = false;
                            break;
                        }
                    }, 
                    "blue" => {
                        if val.parse::<usize>().unwrap() > limit[2] {
                            possible = false;
                            break;
                        }
                    },
                    _ => continue, 
                }
            }
            if !possible {
                break;
            }
        }

        if possible {
            sum += i + 1;
        }
    }
    return sum;
}

fn get_power(lines: Vec<String>) -> usize {

    let mut sum: usize = 0;
    for game in lines.iter() {
        let game_res: &str = game.split(':').collect::<Vec<_>>()[1].trim();

        let draws: Vec<&str> = game_res.split(';').map(|a| a.trim()).collect();

        let vals: Vec<Vec<&str>> = draws.iter()
            .map(|a| a.split(',')
                 .map(|b| b.trim())
                 .collect())
            .collect();
        
        let mut min_limit: [usize; 3] = [0, 0, 0];
        for draw in vals.iter() {
            for val in draw.iter() {

                let [val, col] = val.split(' ').collect::<Vec<_>>().try_into().unwrap();
                match col {
                    "red" => {
                        let v = val.parse::<usize>().unwrap();
                        if v > min_limit[0] {
                            min_limit[0] = v;
                        }
                    },
                    "green" => {
                        let v = val.parse::<usize>().unwrap();
                        if v > min_limit[1] {
                            min_limit[1] = v;
                        }
                    }, 
                    "blue" => {
                        let v = val.parse::<usize>().unwrap();
                        if v > min_limit[2] {
                            min_limit[2] = v;
                        }
                    },
                    _ => continue, 
                }
            }
        }

        sum += min_limit.iter().product::<usize>();
    }
    return sum;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &str = &args[1];
    //println!("{:?}", read_file_lines(path));
    let lines = read_file_lines(path);


    if PART == 1 {
        println!("Sum is {}", get_sum(lines));

    } else if PART == 2 {
        println!("Sum is {}", get_power(lines));
    }
}
