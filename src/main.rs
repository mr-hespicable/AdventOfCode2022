use std::{fs::{self, File}, path::Path, char, process::exit, time::Instant, thread, time};
use clearscreen::clear;

fn main(){
    let mut fnlist: Vec<String> = Vec::new();
    fnlist.push(day_1());
    fnlist.push(day_2());
    fnlist.push(day_3());
    fnlist.push(day_4());
    fnlist.push(day_5());

    for i in 0..fnlist.len(){
        println!("day {}:\n{}\n", i+1, fnlist[i]);
        // println!("elapsed time: {:.2?}", before.elapsed());
    }
}

fn day_1() -> String {
    let before = Instant::now();

    let path = Path::new("inputs/input_day1.txt");
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
    format!("part 1: {}\npart 2: {}", inventory_vec[0], total)
}

fn day_2() -> String {
    let before = Instant::now();

    let path = Path::new("inputs/input_day2.txt");
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
    let before = Instant::now();

    const VALUE_LIST: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let path = Path::new("inputs/input_day3.txt");
    let input = fs::read_to_string(path).expect("file not found");

    let p1: String;
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

        //println!("elf1: {}\nelf2: {}\nelf3: {}", elf1, elf2, elf3);

        let mut inv_set: Vec<char> = Vec::new();

        for char in elf1.chars() {
            match elf2.find(char) {
                None => continue,
                Some(index) => {
                    inv_set.push(elf2.chars().nth(index).unwrap());
                }
            }
        }

        // for t in &inv_set {
            // print!("{}", t);
        // }

        for char in inv_set {
            match elf3.find(char) {
                None => {
                    continue;
                },
                #[allow(unused_variables)]
                Some(rah) => {
                    let matched_char = char;
                    //println!("matched_char: {}", matched_char);
                    let value = (VALUE_LIST.find(matched_char).unwrap() as i32) + 1;
                    //println!("value: {}\n", value);
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

fn day_4() -> String {
    let before = Instant::now();

    let path = Path::new("inputs/input_day4.txt");
    let input = fs::read_to_string(path).expect("file not found");

    let mut containment_count: i32 = 0;
    let mut overlap_count: i32 = 0;

    for line in input.lines() {
        if line == "exit" {
            // println!("count: _}", containment_count);
            exit(0);
        }
        //x-y,a-b
        let pairs: Vec<&str> = line.split(",").collect(); //'x-y', 'a-b'
        let pair_1: Vec<i32> = pairs[0].split("-")
            .map(|s| s.parse().expect("failed integer parsation"))
            .collect();
        let pair_2: Vec<i32> = pairs[1].split("-")
            .map(|s| s.parse().expect("failed integer parsation"))
            .collect();

        let range_1 = pair_1[0]..=pair_1[1];
        let range_2 = pair_2[0]..=pair_2[1];

        let front_containment = range_1.contains(range_2.start()) && range_1.contains(range_2.end());
        let back_containment = range_2.contains(range_1.start()) && range_2.contains(range_1.end());
        
        let front_overlap = range_1.contains(range_2.start()) || range_1.contains(range_2.end());
        let back_overlap = range_2.contains(range_1.start()) || range_2.contains(range_1.end());


        let containment_condition = front_containment || back_containment;
        let overlap_condition = front_overlap || back_overlap;

        if containment_condition {
            // println!("pairs: {:?}\na in b: {}\nb in a: {}", pairs, front_containment, back_containment);
            // println!("range 1: {:?}\nrange 2: {:?}\n\n", range_1, range_2);
            containment_count += 1;
        }

        if overlap_condition {
            overlap_count += 1;
        }
    }

    let p1 = containment_count.to_string();
    let p2 = overlap_count.to_string();

    let result = format!("part 1: {}\npart 2: {}", p1, p2);
    result
}

fn day_5() -> String {
    let before = Instant::now();

    let path = Path::new("inputs/input_day5.txt");
    let input = fs::read_to_string(path).expect("file not found");

    let empty = "   ";
    let empty_vec: Vec<String> = vec![empty.to_string(); 9];

    let mut table_finished: bool = false;
    let mut stack: Vec<Vec<String>> = Vec::new(); //stack[row][column]

    let mut index_counter = 0;
    for raw_line in input.lines() {
        index_counter += 1;
        if raw_line.is_empty() { //check if table is done
            table_finished = true; 
            stack.pop(); //delete empty line, which is due to number list on bottom
            for row in &stack {
                println!("{:?}", row);
            }
            continue
        }

        if table_finished { //do stack things
            //move x from y to z
            let line: Vec<&str> = raw_line.split(" ").collect();
            let amount: usize = line[1].parse().unwrap(); //amount of krates to move
            let initial_column: usize = line[3].parse().unwrap(); //initial krate column
            let final_column: usize = line[5].parse().unwrap(); //final krate column

            let mut output = Path::new(".output");

            let mut initial_coords: (usize, usize) = (0, 0); //row, col
            let mut initial_value: String = "".to_string(); //3 chars

            let mut write_coords: (usize, usize) = (0, 0); //row, col
            //let mut write_value: &String; //3 chars

            //remove empty rows
            stack.retain(|row| {
                !row.iter().all(|krate| krate == "   ")
            });



            //amount of times swapper is run
            for time in 0..amount {
                //insert empty row at the top of the stack
                //this adds lots of padding (is removed after loop)
                stack.insert(0, empty_vec.clone());

                //swapper
                
                //find initial value
                for row in 1..stack.len() {
                    if &stack[stack.len()-row][initial_column-1] != "   " /*&& row-1 != 0*/ {
                        initial_coords = (stack.len()-row, initial_column-1);
                        initial_value = stack[initial_coords.0][initial_coords.1].clone();
                    }
                }

                //find final coords
                for row in 1..stack.len() {
                    if &stack[stack.len()-row][final_column-1] != "   " {
                        write_coords = (stack.len() - (row+1), final_column-1);
                        //write_value = &stack[write_coords.0][write_coords.1]; //should always be "   "
                    }
                }

                //DEBUGGING HERE TO CHECK SOME STUFF 2
                let _ = clear();
                println!("{}", index_counter);

                for row in &stack {
                    let row_str = format!("{:?}", row);
                    let _ = fs::write(output, row_str);
                }
                // let debug_str = format!("amount: {}\ninstruction: {:?}\ninitial value: {:?} at {:?}\nfinal coords: {:?}\noffset: {}", amount, line, initial_value, initial_coords, write_coords, time);
                // let _ = fs::write(output, &debug_str);
                //thread::sleep(time::Duration::from_millis(5000));
                //DEBUGGING END
                
                //move initial value to final coords
                stack[write_coords.0][write_coords.1] = initial_value.clone();

                //erase initial value
                stack[initial_coords.0][initial_coords.1] = empty.to_string();


            }

            //OLD IMPLEMENTATION

            /*
            //stack coords go stack[col][row]
            for i in 1..stack.len() { //in each row: find 
                initial_coords = (i, initial_column-1); //tuple of row: usize, col: usize
                initial_value = stack[initial_coords.0][initial_coords.1].clone(); //is correct(i think)

                if !(initial_value == "   ") && initial_coords.0 != 0 { //
                    for j in 1..stack.len() { //find where krate is gonna land // row
                        write_coords = (j, final_column-1); //row, col
                        write_value = stack[write_coords.0][write_coords.1].clone();

                        if !(write_value == "   ") { 
                            for k in 0..amount {
                                initial_coords = (i+k, initial_column-1); //tuple of row: usize, col: usize
                                initial_value = stack[initial_coords.0][initial_coords.1].clone(); //is correct(i think)
                                //println!("j is {}", j);
                                 
                                //DEBUGGING HERE TO CHECK SOME STUFF 2
                                let _ = clear();
                                for row in stack.clone() {
                                    println!("{:?}", row);
                                }
                                println!("amount: {}", amount);
                                println!("instruction: {:?}", line);
                                println!("initial value: {:?} at {:?}\nfinal value: {:?} at {:?}", initial_value, initial_coords, write_value, write_coords, );
                                println!("offset: {}", k);
                                thread::sleep(time::Duration::from_secs(5));
                                //DEBUGGING END
                                
                                //write initial value to final coordinates
                                stack[write_coords.0-1][write_coords.1] = initial_value.clone();

                                //remove the initial value
                                stack[initial_coords.0][initial_coords.1] = "   ".to_string();
                            }
                            break;
                        }
                    }
                    break;
                }
            }
            */

        } else { //build table
            let mut row: Vec<String> = Vec::new();
            for index in (0..raw_line.chars().count()).step_by(4) {
                let char = raw_line.chars().nth(index).unwrap();
                if char == '[' {
                    row.push(format!("{}{}{}", raw_line.chars().nth(index).unwrap(), raw_line.chars().nth(index+1).unwrap(), raw_line.chars().nth(index+2).unwrap()));
                } else {
                    row.push(empty.to_string());
                }
            }
            stack.push(row);
        }
    }

    //format result

    let mut result = String::new();
    for row in stack.iter() {
        for col in row.iter() {
            result.push_str(col);
            result.push_str("|");
        }
        result.push_str("\n");
    }

    result

}

// pairs: ["4-19", "19-67"]
// cond2: true
// cond1: false
