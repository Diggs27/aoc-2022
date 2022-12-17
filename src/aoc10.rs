use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

enum instruciton{
    noop,
    addx
}
pub fn run () {

    let mut cycle: i32 = 0;
    let mut register: i32 = 1;
    let mut total: i32 = 0;
    let mut line_pos: i32 = 0;
    let mut display_line: String = String::from("Blash");

    if let Ok(lines) = read_lines("aoc10_input"){
        let mut row: i32 = 0;
            for line in lines{
            if let Ok(instruction_line) = line {

                let instruct = instruction_line.split(" ").collect::<Vec<_>>();
                let instruct_type = instruct[0];
                
                if instruct.len() > 1 {
                    cycle = cycle +1; 
                    total = cycle_val(cycle, total,register);
                    cycle = cycle +1; 
                    total = cycle_val(cycle, total,register);
                    register = register + instruct[1].parse::<i32>().unwrap();
                    
                } else {
                    cycle = cycle + 1;
                    total = cycle_val(cycle, total,register);
                }
              
                if cycle % 40 == 0 {
                    println!("{}",display_line);
                    display_line = String::from("Blah");
                    line_pos = 0;
                }

            }
            
         }
         println!("The Total is {}", total);
    }


}



fn cycle_val(cycle: i32, mut total: i32, register: i32) -> i32{
    match cycle {
        20 => total = total + (20 * register), 
        60 => total = total + (60 * register), 
        100 => total = total + (100 * register), 
        140 => total = total + (140 * register), 
        180 => total = total + (180 * register), 
        220 => total = total + (220 * register), 
        _ => { }
    }
    return total
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

