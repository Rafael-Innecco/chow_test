use std::fs;
use std::env;
use std::error::Error;
use crate::stat::*;

pub mod dp_vec;
pub mod stat;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let file_name = &args[1];
    let break_index = args[2].parse::<i32>().unwrap();

    let all_data = read_csv(file_name, break_index).unwrap();

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

#[derive(Debug, serde::Deserialize)]
struct Record {
    pub date: String,
    pub close: Option<f64>
}

fn read_csv(file_name: &str, break_index: i32) -> Result<Vec<Vec<(f64, f64)>>, Box<dyn Error>> {
    let file = fs::File::open(file_name)
        .expect("Something went wrong when reading the file");
    let mut rdr = csv::ReaderBuilder::new().delimiter(b',').from_reader(file);
    let mut data_vec: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut i: i32 = 0;
    let mut data_vec_1 = Vec::new();
    let mut data_vec_2 = Vec::new();
    
    println!("Start reading csv");
    
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        println!("{:?}", result);
        let record: Record = result?;
        let new_data = (i as f64, record.close.unwrap());
        if i < break_index {
            data_vec_1.push(new_data);
        } else {
            data_vec_2.push(new_data);
        }

        i = i + 1;
    }

    data_vec.push(data_vec_1);
    data_vec.push(data_vec_2);

    return Ok(data_vec);
}