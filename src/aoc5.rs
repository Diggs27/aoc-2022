use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn run () {
  
    let starting_row: usize = 8;
    let mut total: i32 = 0;
    let mut sacks_of_chars: Vec<Vec<char>> = Vec::new();
    if let Ok(mut lines) = read_lines("aoc5_input"){
        let line = lines.nth(starting_row);

        if let Some(Ok(l)) = line {
            println!("The ninth line {}",l);
            for (i, c) in l.chars().enumerate() {
                if c.is_numeric(){
                    println!("{} {}",i,c);
                    total = total + 1;
                    let mut row_items: Vec<char> = Vec::new();
                    for n in (0..8).rev() {
                        let detail_line = get_line(n);
                        let item= detail_line.chars().nth(i).unwrap();
                        
                        if item.is_alphabetic(){
                            println!("Line {}", item);
                            row_items.push(item);
                        }
                        
                       
                    }
                    sacks_of_chars.push(row_items);
                }
               
            }
            let parts_vec = l.split(" ").collect::<Vec<_>>().len();
            println!("The count is {}",parts_vec)
            
        }


    }
    //print_vec_items(sacks_of_chars);

    //Now lets walk through the instructions to move the stack
    let mut starting_instruction_line: i32 = 10;
    //Start walking through the lines again
    
    if let Ok(mut lines) = read_lines("aoc5_input"){
        for i in (0..starting_instruction_line){
            lines.next();
        }
        for line in lines{
            if let Ok(instruction_line) = line {
                println!("{}",instruction_line);
                let instruction_parts = instruction_line.split(" ").collect::<Vec<_>>();
                let num_of_items_to_move: i32 = instruction_parts[1].parse::<i32>().unwrap();
                let from_item: i32 = instruction_parts[3].parse::<i32>().unwrap() - 1;
                let to_item: i32 = instruction_parts[5].parse::<i32>().unwrap() -1;

                println!("{} {} {}",num_of_items_to_move,from_item,to_item);
                let mut temp_vec: Vec<char> = Vec::new();
                
                for i in (0..num_of_items_to_move){
                    
                    let it: char = sacks_of_chars[from_item as usize].pop().unwrap();
                    temp_vec.insert(0,it);
                }
                for temp in temp_vec{
                    sacks_of_chars[to_item as usize].push(temp);
                    println!("{}",temp);
                }

                
                

                
            }
        }
        print_vec_items(sacks_of_chars);
    }
    

}

fn print_vec_items(input: Vec<Vec<char>>){
      let mut total: i32 = 1;
      for j in input.iter(){
        println!("Items in sack {}: ",total);
        for x in j.iter(){
            print!(" {} ",x)
        }
        total = total +1;
      }
}

fn get_line(num: usize)-> String{
    if let Ok(mut lines) = read_lines("aoc5_input"){
        let line = lines.nth(num);

        if let Some(Ok(l)) = line {
            return l;
        }
        else{
            return String::from("");
        }
    }
    return String::from("");
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
