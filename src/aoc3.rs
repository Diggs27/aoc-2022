use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn run () {
    let r: i32 = run_aoc3_part2();
    println!("Total {}", r)
}
pub fn run_aoc3_part2() -> i32 {
    let mut result: i32 = 0;
    let mut total: i32 = 0;
    if let Ok(mut lines) = read_lines("./aoc3_input") {
        // Consumes the iterator, returns an (Optional) String
        while let(line1,line2,line3) = (lines.next(), lines.next(), lines.next()){
              if line1.is_none() {
                break;
              }
              let mut line_one: String = String::new();
              let mut line_two: String = String::new();
              let mut line_three: String = String::new();
              if let Some(Ok(line)) =line1{
                line_one = line;
                println!("{}",line_one)
              }
              if let Some(Ok(line)) =line2{
                line_two = line;
              }
              if let Some(Ok(line)) =line3{
                line_three = line;
              }

              for c in line_one.chars() {
                  if line_two.contains(c) && line_three.contains(c){
                    total = total + get_char_total(c);
                    println!("Found: {}", c);
                    break;
                  }
              }
                  
              
        }
        


  

        
    }
    return total;
}

fn get_char_total(input: char) -> i32 {
    let mut s: i32 = 0;
    if(input.is_uppercase())  {
        s = input as i32 -64 +26;
    }
    if(input.is_lowercase())  {
        s = input as i32 -96;
    }
    return s;
}

pub fn run_aoc3()-> i32{
    let mut result: i32 = 0;
    let mut total: i32 = 0;
    if let Ok(lines) = read_lines("./aoc3_input") {
        // Consumes the iterator, returns an (Optional) String
        let mut count: i32 = 0;
        

    
        for line in lines {
            if let Ok(ip) = line {
                let mut bag_size = ip.chars().count() / 2;
                let mut overlapping_chars: Vec<char> = vec![];
                
                let part_1: &str = &ip[..bag_size];
                let part_2: &str = &ip[bag_size..];
                println!("Part 1 {} 2 {}", &ip[..bag_size],&ip[bag_size..]);

                for i in part_1.chars(){
                    if part_2.contains(i) && !overlapping_chars.contains(&i) {
                        overlapping_chars.push(i);
                        if(i.is_uppercase())  {
                            println!("{}", i);
                            println!("{}", i as i32 -64 +26);
                            let s: i32 = i as i32 -64 +26;
                            total = total + s;
                            println!("Total {}", total);
                        }
                        if(i.is_lowercase())  {
                            println!("{}", i);
                            println!("{}", i as i32 -96);
                            let s: i32 = i as i32 -96;
                            total = total + s;
                        }
                     
                    }
                    
                }

                
      
              

            }

        
        }
    }
    return total;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
