use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn run () {
    run_aoc3();
}

pub fn run_aoc3()-> i32{
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines("./aoc3_input") {
        // Consumes the iterator, returns an (Optional) String
        let mut count: i32 = 0;
        let mut total: i32 = 0;

    
        for line in lines {
            if let Ok(ip) = line {
                let mut bag_size = ip.chars().count() / 2;
                println!("Length {}", bag_size);

                
      
              

            }

        
        }
    }
    return result;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
