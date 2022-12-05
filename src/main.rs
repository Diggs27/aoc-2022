use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod aoc2;

fn main() {
    aoc2::run();
}

fn aoc2(){
    if let Ok(lines) = read_lines("./aoc2") {
        // Consumes the iterator, returns an (Optional) String
        let mut count: i32 = 0;
        let mut total: i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let split = ip.split(" ");
                let vec = split.collect::<Vec<&str>>();
                
                let r: i32 = get_test(vec[0], vec[1]);
                println!("{} {} = {} ", vec[0], vec[1], r);
                
                total = total + r;
              

            }
         count = count + 1;
        
        }
        println!("Total {}", total);
    }
}

fn get_test(opp: &str, play: &str) -> (i32) {
    let mut res: i32 = 0;
    match opp {
        "A" => { match play { 
            "X" => res = 0 + 3, 
            "Y" => res = 3 + 1,
            "Z" => res = 6 + 2,
            _ => println!("Ain't special"),
            };
        }
        "B" => { match play { 
            "X" => res = 0 + 1,
            "Y" =>  res = 3  + 2,
            "Z" => res =6  + 3,
            _ => println!("Ain't special"),
            };
        }
        "C" => { match play { 
            "X" =>  res = 0 + 2,
            "Y" =>  res = 3 + 3,
            "Z" =>  res = 6 + 1, 
            _ => println!("Ain't special"),
            };
        }
        _ => println!("Ain't special"),
    }
    return res;
}

fn aoc1(){
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