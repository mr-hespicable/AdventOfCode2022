use std::{io, env, fs::{self, File, DirEntry}, collections::HashMap, path::Path, char, str::FromStr};

fn main(){
    println!("What day would you like to run?");
    let mut day = String::from("day_");
    io::stdin().read_line(&mut day).expect("ruh roh");
    println!("{}", day.as_str());
match day.as_str() {
        "day_1\n" => day_1(),
        "day_2\n" => day_2(),
        "day_3\n" => day_3(),
        _ => println!("whoops that one doesn't exist yet"),
    }
} 

fn day_1() {
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

fn day_2() {
    let path = Path::new("../inputs/input_day2.txt");
    let input = fs::read_to_string(path).expect("file not found");
    //A = Rock: 1
    //B = Paper: 2
    //C = Scissors: 3
    let mut score: i32 = 0;
    for line in input.lines() {
        let vals: Vec<&str> = line.split(" ").collect();
        let opponent: char = vals[0].parse().unwrap();
        let response: char = vals[1].parse().unwrap();

        score += game_eval(opponent, response);
    }

    println!("{score}");

    fn game_eval(op: char, rp: char) -> i32 {
        if (op == 'A' && rp == 'Z') || (op == 'B' && rp == 'Z') || (op == 'C' && rp == 'Z') { //win
            let mut result: i32 = 6;
            match op {
                'A' => result += 2,
                'B' => result += 3,
                'C' => result += 1,
                _ => panic!("uh oh"),
            }
            result

        } else if (op == 'A' && rp == 'Y') || (op == 'B' && rp == 'Y') || (op == 'C' && rp == 'Y') { //tie
            let mut result: i32 = 3;
            match op {
                'A' => result += 1,
                'B' => result += 2,
                'C' => result += 3,
                    _ => panic!("uh oh"),
            }
            result

        } else if (op == 'A' && rp == 'X') || (op == 'B' && rp == 'X') || (op == 'C' && rp == 'X') { //loss
            let mut result: i32 = 0;
            match op {
                'A' => result += 3,
                'B' => result += 1,
                'C' => result += 2,
                    _ => panic!("uh oh"),
            }
            result

        } else {
            panic!("one of ur inputs are wrong");
        }
    }
}

fn day_3() {
    const VALUE_LIST: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let path = Path::new("../inputs/input_day3.txt");
    let input = fs::read_to_string(path).expect("file not found");

    let mut priority_sum = 0;

    for line in input.lines() {
        let (cpart1, cpart2) = line.split_at(line.chars().count()/2);
        println!("p1 = {cpart1}\np2 = {cpart2}");
        for char in cpart1.chars() {
            let index = cpart2.find(char).unwrap();
            let value = VALUE_LIST.find(cpart2.chars().nth(index).unwrap()).unwrap() + 1;
            println!("value is {value}");
            priority_sum += value;
        }
    }
    println!("psum is {priority_sum}");
}

