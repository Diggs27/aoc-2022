use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn run () {
  
    if let Ok(mut lines) = read_lines("aoc6_input"){
        let line = lines.nth(0);
        let mut code: Vec<char> = Vec::new();
        if let Some(Ok(l)) = line {
            println!("Line {}", l);
            for (i,ch) in l.chars().enumerate(){
                if !code.contains(&ch){
                    code.push(ch);
                }
                else{
                    let pos: usize = code.iter().position(|&x| x == ch).unwrap();
                    println!("{}", pos);
                    //code = Vec::new();
                    let next: usize = pos+1;
                    code = code[next ..code.len()].to_vec();
                    code.push(ch);
                }
                if code.len() == 14 {
                    println!("{} {}", i, ch);
                    break;
                }
                println!("{} {}", i, ch);
            }
        }


    }
  
    

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
