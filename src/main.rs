use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let mut counter: i32 = 0;
    let mut total: i32 = 0;
    let mut highest: i32 =0;
    if let Ok(lines) = read_lines("./aoc1") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);         
                match ip.parse::<i32>() {
                    Ok(n) => {
                        total = total + n;
                    },
                    Err(e) => {
                        vec.push(total);
                        if highest < total{
                            highest = total;
                        }
                        total = 0;
                        counter = counter +1; 
                    },
                  }
            }
        }
    }
    vec.sort();
    for item in vec.iter(){
        println!("Total {}", item);
    }
    println!("highest {}",highest)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn add_to_list(line_number: i32){

}

fn int_exception(){
    println!("Invalid Int");   
}