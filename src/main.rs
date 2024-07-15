use std::fs;
use std::env;
use crate::stat::*;

pub mod dp_vec;
pub mod stat;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let file_name = &args[1];

    let all_data = read_data(file_name);

    let _p = chow_test(all_data);
}

fn read_data(file_name: &str) -> Vec<Vec<(f64, f64)>> {
    let mut v: Vec<Vec<(f64, f64)>> = Vec::new();

    let file_contents = fs::read_to_string(file_name)
        .expect("Something went wrong when reading the file");

    let lines: Vec<&str> = file_contents.split("\n").collect::<Vec<&str>>();

    for line in lines {
        let trimmed_line: String = line.chars().filter(|c| !c.is_whitespace()).collect();

        if trimmed_line.is_empty() {
            continue;
        }
        
        let data_point_strings: Vec<String> = trimmed_line.split(";").collect::<Vec<&str>>()
            .iter().map(|s| s.replace(&['(', ')'][..], "")).collect();
        
        let data_points: Vec<(f64, f64)> = data_point_strings.iter().map(|s| string_to_tuple(s)).collect();

        v.push(data_points);
    }
    
    v
}


fn string_to_tuple(s: &str) -> (f64, f64) {
    let split_string: Vec<&str> = s.split(",").collect();

    return (split_string[0].trim().parse::<f64>().unwrap(), split_string[1].trim().parse::<f64>().unwrap());
}
