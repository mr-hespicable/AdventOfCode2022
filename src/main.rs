use std::{io, env, fs::{self, File, DirEntry}, collections::HashMap, path::Path, char, str::FromStr, fmt::format};

fn main(){
    let mut fnlist: Vec<String> = Vec::new();
    fnlist.push(day_1());
    fnlist.push(day_2());
    fnlist.push(day_3());

    for i in 0..fnlist.len(){
        println!("Day {}:\n{}\n", i+1, fnlist[i]);
    }
}

fn day_1() -> String {
    let path = Path::new("./inputs/input_day1.txt");
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
    format!("1. {} (part 1)\n2. {}\n3. {}\ntotal: {}", inventory_vec[0], inventory_vec[1], inventory_vec[2], total)
}

fn day_2() -> String {
    let path = Path::new("./inputs/input_day2.txt");
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
    let p1 = (11386).to_string();
    let p2 = score.to_string();

    let result = format!("part 1: {}\npart 2: {}", p1, p2);
    result
}

fn day_3() -> String {
    const VALUE_LIST: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let path = Path::new("./inputs/input_day3.txt");
    let input = fs::read_to_string(path).expect("file not found");

    let mut p1: String;
    let mut p2: i32 = 0;

    //part 1
    let mut priority_sum = 0;

    for line in input.lines() { 
        let (cpart1, cpart2) = line.split_at(line.chars().count()/2);
        //println!("p1 = {cpart1}\np2 = {cpart2}");
        for char in cpart1.chars() {
            //println!("char: {char}");
            match cpart2.find(char) {
                None => continue,
                Some(index) => {
                    //println!("index: {index}");
                    let value = VALUE_LIST.find(cpart2.chars().nth(index).unwrap()).unwrap() + 1; //usize
                    //println!("value: {value}\n");
                    priority_sum += value as i32; //add value to priority sum
                    break; //break out of loop
                }
            }
        }
    }

    //part 2
    let mut lines: Vec<&str> = Vec::new();
    for line in input.lines() {
        lines.push(line);
    }

    for i in 0..(lines.len()/3) {
        let start_index = i*3;

        let elf1 = lines[start_index];
        let elf2 = lines[start_index+1];
        let elf3 = lines[start_index+2];

        println!("elf1: {}\nelf2: {}\nelf3: {}", elf1, elf2, elf3);

        let mut inv_set: Vec<char> = Vec::new();

        for char in elf1.chars() {
            match elf2.find(char) {
                None => continue,
                Some(index) => {
                    inv_set.push(elf2.chars().nth(index).unwrap());
                }
            }
        }

        print!("inv_set: ");
        for t in &inv_set {
            print!("{}", t);
        }
        println!("\n");

        for char in inv_set {
            match elf3.find(char) {
                None => {
                    continue;
                },
                #[allow(unused_variables)]
                Some(rah) => {
                    let matched_char = char;
                    println!("matched_char: {}", matched_char);
                    let value = (VALUE_LIST.find(matched_char).unwrap() as i32) + 1;
                    println!("value: {}\n", value);
                    p2 += value;
                    break;
                },
            }
        }
    }
    
    p1 = priority_sum.to_string();

    let result = format!("part 1: {}\npart 2: {}", p1, p2);
    result
}

//whoa.
