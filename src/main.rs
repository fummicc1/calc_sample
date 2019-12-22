use std::env;
use std::io;
use std::convert::From;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let arg = &args.first().unwrap();
    let value = parse_int(arg);

    if value.is_none() { 
        let _value = get_input();
    };

    let arg2 = &args.get(1).unwrap();
    let value2 = parse_int(arg2);


    println!("Inputs: {:?}", args);
}

fn get_input() -> Option<i32> {
    let text: String;

    match io::stdin().read_line(&mut text) {
        Ok(s) => {
            let t = s.to_string()[..];
            Some(parse_int(t).unwrap())
        },
        Err(error) => {
            None
        }
    }
}

// parse_int tells caller spcified error (ParseIntError)
fn parse_int(number_str: &str) -> Option<i32> {
    match number_str.parse::<i32>() {
        Ok(number) => Some(number),
        Err(err) => {
            println!("error: {:?}", err);
            None
        },
    }
}

fn increment(x: i32, y:i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}
