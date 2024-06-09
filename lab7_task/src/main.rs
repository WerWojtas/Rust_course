use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;
 
const DATA_FILE : &str = "data.json";
 
const API_QUERY : &str = "https://api.openf1.org/v1/car_data?driver_number=55&session_key=9159";
 
#[derive(Serialize, Deserialize, Debug)]
struct CarData {
    brake : u32,
    date: DateTime<Utc>,
    driver_number: u32,
    drs: u8,
    meeting_key: u32,
    n_gear: u8,
    rpm: u32,
    session_key: u32,
    speed: u32,
    throttle: u8
}

 
fn fetch_car_data(url: &str, data_file : &str) -> Result<()> {
    let data = reqwest::blocking::get(url)?.text()?;
    let car_data : Vec<CarData> = serde_json::from_str(&data)?;
    let writer = BufWriter::new(File::create(data_file)?);
    serde_json::to_writer(writer, &car_data)?;
    Ok(())
}
 
fn load_car_data_from_file(data_file : &str) -> Result<Vec<CarData>> {
    let car_data : Vec<CarData> = serde_json::from_reader(BufReader::new(File::open(data_file)?))?;
    Ok(car_data)
}
 
fn check_if_data_exists(data_file : &str) -> bool {
    Path::new(data_file).exists()
}

fn average_speed(car_data : &Vec<CarData>, driver_number : u32, session_key : u32) -> f32 {
    let mut sum = 0;
    let mut count = 0;
    for data in car_data {
        if data.driver_number == driver_number && data.session_key == session_key {
            sum += data.speed;
            count += 1;
        }
    }
    if count == 0 {
        return 0.0;
    }
    sum as f32 / count as f32
}

fn max_rpm(car_data : &Vec<CarData>, driver_number : u32) -> (u32, u8) {
    let mut max = 0;
    let mut gear = 0;
    for data in car_data {
        if data.driver_number == driver_number {
            if data.rpm > max {
                max = data.rpm;
                gear = data.n_gear
            }
        }
    }
    (max, gear)
}
 
fn main() -> Result<()> {
 
    if !check_if_data_exists(DATA_FILE) {
        fetch_car_data(API_QUERY, DATA_FILE)?;
    }
 
    let car_data = load_car_data_from_file(DATA_FILE).unwrap();
    
    let sum = average_speed(&car_data, 55, 9159);
    let max = max_rpm(&car_data, 55);
    println!("{:?}", sum);
    println!("{:?}", max);
    
    Ok(())
}
 
 