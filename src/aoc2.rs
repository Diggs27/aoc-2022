

    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    pub enum Stratagy{
        ElfStratagyOne,
        ElfStratagyTwo
    }

    pub fn run () {
         let ans_1: i32;
         let ans_2: i32;
         ans_1 = get_result(Stratagy::ElfStratagyOne);
         ans_2 = get_result(Stratagy::ElfStratagyTwo);
         println!("The answer for part 2 is {}", ans_1);
         println!("The answer for part 2 is {}", ans_2)
    }

    pub fn get_result(strat: Stratagy)-> i32{
        let mut result: i32 = 0;
        if let Ok(lines) = read_lines("./aoc2_input") {
            // Consumes the iterator, returns an (Optional) String
            let mut count: i32 = 0;
            let mut total: i32 = 0;
            for line in lines {
                if let Ok(ip) = line {
                    let split = ip.split(" ");
                    let vec = split.collect::<Vec<&str>>();
                    let mut r: i32 = 0;
                    match strat {
                        Stratagy::ElfStratagyOne => { r = strategy_one(vec[0], vec[1]) },
                        Stratagy::ElfStratagyTwo => { r = strategy_two(vec[0], vec[1]) },
                    }
                    println!("{} {} = {} ", vec[0], vec[1], r);
                    
                    total = total + r;
                  
    
                }
             count = count + 1;
            
            }
            result = total;
        }
        return result;
    }

    fn strategy_one(opp: &str, play: &str) -> (i32) {
        let mut res: i32 = 0;
        match opp {
            "A" => { match play { 
                "X" => res = 3 + 1, 
                "Y" => res = 6 + 2,
                "Z" => res = 0 + 3,
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
                "X" =>  res = 6 + 1,
                "Y" =>  res = 0 + 2,
                "Z" =>  res = 3 + 3, 
                _ => println!("Ain't special"),
                };
            }
            _ => println!("Ain't special"),
        }
        return res;
    }

    
    fn strategy_two(opp: &str, play: &str) -> (i32) {
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


    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    
