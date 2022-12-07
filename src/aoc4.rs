use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn run () {
   
    let r: i32 = run_aoc4_part1();
    println!("Total {}", r);
}
pub fn run_aoc4_part1() -> i32 {
    let mut result: i32 = 0;
    let mut total: i32 = 0;
    let mut part2_total: i32 = 0;
  
    if let Ok(lines) = read_lines("aoc4_input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(ip) = line {
                let parts_vec = ip.split(",").collect::<Vec<_>>();
                let part1 = parts_vec[0];
                let part2 = parts_vec[1];
                let part1_vec = part1.split("-").collect::<Vec<_>>();
                let part1_start = part1_vec[0].parse::<i32>().unwrap();;
                let part1_end = part1_vec[1].parse::<i32>().unwrap();;
                let part2_vec = part2.split("-").collect::<Vec<_>>(); 
                let part2_start = part2_vec[0].parse::<i32>().unwrap();;
                let part2_end = part2_vec[1].parse::<i32>().unwrap();;

                if (part1_start >= part2_start && part1_end <= part2_end) || (part2_start >= part1_start && part2_end <= part1_end) {
                    println!("Found contains with {} and {}",part1, part2);
                    total = total + 1;
                }
                if (part2_start >= part1_start && part2_start <= part1_end) || (part1_start >= part2_start && part1_start <= part2_end) {
                    println!("Found overlap with {} and {}",part1, part2);
                    part2_total = part2_total + 1;
                }
                
                
                
            }
            
        }
            
        
    }
    return part2_total;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
