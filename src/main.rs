use std::{io, env, fs::{self, File, DirEntry}, collections::HashMap, path::Path};

fn main(){
    println!("What day would you like to run?");
    let mut day = String::from("day_");
    io::stdin().read_line(&mut day).expect("ruh roh");
    println!("{}", day.as_str());

    match day.as_str() {
        "day_1\n" => day_1(),
        _ => println!("whoops that one doesn't exist yet"),
    }
} 

fn day_1(){
    let path = Path::new("../inputs/input_day1.txt");
    let input = fs::read_to_string(path).expect("file not found");

    let mut inventory_vec: Vec<i32> = Vec::new();

    let mut cals: i32 = 0;

    //add list of inventory total cals
    for line in input.lines() {
        if line.is_empty() {
            inventory_vec.push(cals);
            cals = 0
        } else {
            cals += line.parse::<i32>().unwrap();
        }
    }

    //sort & flip the vector
    inventory_vec.sort();
    inventory_vec.reverse();

    let total = inventory_vec[0] + inventory_vec[1] + inventory_vec[2];

    println!("1. {}\n2. {}\n3. {}\ntotal: {}", inventory_vec[0], inventory_vec[1], inventory_vec[2], total);

}

