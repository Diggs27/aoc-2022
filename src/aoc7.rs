use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


pub fn run () {
    //Holds Values of dir
    let mut cwd = String::from("root");
    let mut dirs: HashMap<String, i32> = HashMap::new();
    dirs.insert(cwd.clone(),0);
    if let Ok(lines) = read_lines("aoc7_input"){

        let mut code: Vec<char> = Vec::new();
            for line in lines{
            if let Ok(command) = line {
                
                let commands = command.split(" ").collect::<Vec<_>>();
                match commands[0] {
                     "$" => {
                        if commands[1] == "cd" {
                            run_command(commands[1].to_string(), commands[2].to_string(), &mut cwd);
                        }
                     }
                     ,
                     _ =>  match commands[0].parse::<i32>() {
                        Ok(n) => { 
                            if dirs.contains_key(&cwd.clone()) {
                                let mut i: &i32 = dirs.get(&cwd.clone()).unwrap();
                                println!("The value is {}", i);
                                *dirs.get_mut(&cwd).unwrap() += n;

                                //Now update all parent directories
                                let v: Vec<_> = cwd.match_indices("\\").collect();
                                let temp_dir: String = cwd.clone();
                                println!("Parent Directories");
                                for (size, value) in v.iter() {
                                        let dir_string = temp_dir.get(0..*size).unwrap().to_string();
                                        let value = *dirs.get(&dir_string).unwrap();
                                        *dirs.get_mut(&dir_string).unwrap() += n;
                                        println!("{} {}",dir_string, value);

                             
                                }





                            } else {
                                dirs.insert(cwd.clone(), n);
                            }
                        
                          },
                        Err(e) => continue,
                      }      
                }
                if ! dirs.contains_key(&cwd.clone()) {
                    dirs.insert(cwd.clone(),0);
                }
            }

         }
        

    }
    println!("-----------------");
    for (key, value) in &dirs {
        println!("{} , {}", key, value);
    }

}

fn run_command(command: String, params: String, cwd: &mut String) {
    let cdv = String::from("cd");
    match command {
        cdv => cd(params, cwd)
    }
    
}


fn cd (arg: String, cwd: &mut String ){
    
    let cd: String = String::from("..");
    if arg == ".." {
        let v: Vec<_> = cwd.match_indices("\\").collect();
        let ind: usize = v.get(v.len()-1).unwrap().0;
        *cwd = cwd.get(0..ind).unwrap().to_string();
        println!("Moving up one directory {}", cwd);
    } else {
      
         cwd.push_str("\\");
         *cwd = cwd.to_owned() + &arg;

         println!("Entering Directory {}", cwd);
    }
}

fn parent(){
    println!("Parent!");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

