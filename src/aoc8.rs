use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

struct Tree {
    row: i32,
    col: i32,
    value: u32,
    visable: Option<bool>
}

pub fn run () {
    const size: usize = 98;
 
    let mut grid: Vec<Vec<Tree>> = Vec::new();
    let mut items : Vec<Vec<u32>> = Vec::new();

    if let Ok(lines) = read_lines("aoc8_input"){
        let mut row: i32 = 0;
            for line in lines{
            if let Ok(tree_line) = line {
                let mut col: i32 = 0;
                let trees = tree_line.chars().collect::<Vec<_>>();
                let mut rows: Vec<Tree> = Vec::new();
                let mut raw_rows: Vec<u32> = Vec::new();
                for (num, tree) in trees.iter().enumerate(){
                    const RADIX: u32 = 10;
                    let tree_int = tree.to_digit(RADIX).unwrap();
                    let tree_st = Tree {
                        row: row,
                        col: col,
                        value: tree_int,
                        visable: None
                    };
                    raw_rows.push(tree_int);
                    rows.push(tree_st);
                    col = col +1;
                }
                items.push(raw_rows);
                grid.push(rows);
                row = row + 1;
            }
            
         }
        
    }

   let mut row: usize = 0;
   let mut col: usize = 0;
   let size_var: usize = grid.len();
   for item in grid.iter_mut(){
        col = 0;
        for tree in item.iter_mut(){
            if row == 0 || col == 0 || row == size_var -1 || col == size_var -1 {
                tree.visable = Some(true);
                //println!(" visable {} {} ", row, col)
            }
            else {
                //println!("Tree value {} for {} {}", items[..row].iter().map(|v| v[col]).max().unwrap() , row, col);
                let l: bool = *items[row][..col].iter().max().unwrap() < tree.value;
                let r: bool = *items[row][col+1..].iter().max().unwrap() < tree.value;
                let t: bool = items[..row].iter().map(|v| v[col]).max().unwrap() < tree.value;
                let b: bool = items[row+1..].iter().map(|v| v[col]).max().unwrap() < tree.value;


                let len= size_var;
                let l:u64 = 1+items[row][1..col].iter().rev().map_while(|&v| if v < items[row][col] { Some(v) } else { None }).count() as u64;
                let r:u64 = 1+items[row][col+1..len-1].iter().map_while(|&v| if v < items[row][col] { Some(v) } else { None }).count() as u64;
                let t:u64 = 1+items[1..row].iter().rev().map(|v| v[col]).map_while(|v| if v < items[row][col] { Some(v) } else { None }).count() as u64;
                let b:u64 = 1+items[row+1..len-1].iter().map(|v| v[col]).map_while(|v| if v < items[row][col] { Some(v) } else { None }).count() as u64;
               

                println!("Max size {}", l*r*t*b);
           
                //if l||r||t||b {
                //    println!(" visable {} {} ", row, col)
                //}
                
                
            }
            
            col += 1;
        }
        row += 1;
   }

}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

