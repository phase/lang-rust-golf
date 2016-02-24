use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

struct Function {
    s: str,
}

impl Function {
    
}

fn exec(c: char) {
    
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        //file
        let path = Path::new(&*args[1]);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };
        
        let mut s = String::new();
        file.read_to_string(&mut s).ok().expect("Failed to read file");
        for (i,c) in s.chars().enumerate() {
            exec(c);
        }
    } else {
        //repl
    }
}