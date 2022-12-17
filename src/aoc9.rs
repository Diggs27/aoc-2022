use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

enum rope_movement{
    right,
    left,
    up,
    down
}
pub fn run () {

    let mut current_position: (i32,i32) = (0,0);
    let mut current_t_position: (i32,i32) = (0,0);
    let mut tails: Vec<(i32,i32)> = Vec::new();

    //Push 0,0 for all starting postions
    for i in 0..9 {
        tails.push((0,0));
    }
    
    if let Ok(lines) = read_lines("aoc9_input"){
        let mut row: i32 = 0;
            for line in lines{
            if let Ok(instruction_line) = line {

                let instruction = instruction_line.split(" ").collect::<Vec<_>>();
                let direction = instruction[0];
                let movement = instruction[1].parse::<i32>().unwrap();

                //println!("Values {} {}", direction, movement);
                let direction_enum = get_direction_enum(direction);

                for moves in 0..movement {
                    current_position = move_item(&direction_enum, current_position);
      
                    //println!("Current H Position {} {}",current_position.0,current_position.1);
                    let mut temp_position = current_position;
                    

                    for i in 0..9 {
                        tails[i] = move_t_item(temp_position, tails[i], &direction_enum);
                        temp_position = tails[i];
                        if i == 8 {
                            println!("Position of {} at {} {}", i, tails[i].0,tails[i].1)
                        }
                        
                    }
                    
                }
                
                
            }
            
         }
        
    }


}

fn get_direction_enum (direction: &str) -> rope_movement {
    match direction {
        "U" => rope_movement::up,
        "D" => rope_movement::down,
        "L" => rope_movement::left,
        "R" => rope_movement::right,
        &_ =>  std::process::exit(-1)
    }
}




fn move_t_item(current_j_position: (i32,i32), current_t_position: (i32,i32), direction: &rope_movement) ->((i32,i32)) {
      let x_deleta: i32 = (current_j_position.0 - current_t_position.0);
      let y_deleta: i32 = (current_j_position.1 - current_t_position.1);

       match (x_deleta,y_deleta) {
         (2,0) => (current_t_position.0+1, current_t_position.1),
         (-2,0) => (current_t_position.0-1, current_t_position.1),
         (0,2) => (current_t_position.0, current_t_position.1+1),
         (0,-2) => (current_t_position.0, current_t_position.1-1),

         (2,1) => (current_t_position.0+1, current_t_position.1+1), 
         (2,-1) => (current_t_position.0+1, current_t_position.1-1), 
         (-2,1) => (current_t_position.0-1, current_t_position.1+1),
         (-2,-1) => (current_t_position.0-1, current_t_position.1-1),  
         (1,2) => (current_t_position.0+1, current_t_position.1+1),   
         (1,-2) => (current_t_position.0+1, current_t_position.1-1),
         (-1,2) => (current_t_position.0-1, current_t_position.1+1), 
         (-1,-2) => (current_t_position.0-1, current_t_position.1-1), 
         (-2,-2) => (current_t_position.0-1, current_t_position.1-1), 

         (-2,2) => (current_t_position.0-1, current_t_position.1+1), 
         (2,2) => (current_t_position.0+1, current_t_position.1+1), 
         (2,-2) => (current_t_position.0+1, current_t_position.1-1), 

         (0,0) => (current_t_position.0, current_t_position.1),
         _ => (current_t_position.0, current_t_position.1)
       }




}




fn move_item(direction: &rope_movement, current_position: (i32,i32)) ->(i32,i32) {
    match direction {
        rope_movement::up  => {
            (current_position.0,current_position.1 +1)
        }
        , 
        rope_movement::down => {
             (current_position.0,current_position.1-1)
        },
        rope_movement::left => {
            (current_position.0-1,current_position.1)
        } ,
        rope_movement::right => {
             (current_position.0+1,current_position.1)
        }
    }
    
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

