use regex::Regex;
use std::{env, fs};

fn get_loadstring(input_code: String) -> String {
    let regex = Regex::new(r#"load(string)?\(("|'|\[\[).+("|'|\]\])\)\(\)"#)
        .expect("Failed to compile regex");
    let mut bytecode = String::new();

    for capture in regex.find(&input_code) {
        for i in capture.start()..capture.end() {
            bytecode.push(
                input_code
                    .chars()
                    .nth(i)
                    .expect(&format!("Failed to get index {} in code string", i)),
            );
        }
        break;
    }

    return bytecode;
}

fn get_bytecode(input: String) -> String {
    let regex = Regex::new(r#"(\\\d+)+"#).expect("Failed to compile regex");
    let mut bytecode_string = String::new();

    for capture in regex.find(&input) {
        for i in capture.start()..capture.end() {
            bytecode_string.push(
                input
                    .chars()
                    .nth(i)
                    .expect(&format!("Failed to get index {} in code string", i)),
            );
        }
    }

    return bytecode_string;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_code;

    if args.len() > 1 {
        input_code =
            fs::read_to_string(&args[1]).expect(&format!("File {} does not exist", args[1]));
    } else {
        input_code = fs::read_to_string("Input.lua").expect("File Input.lua does not exist");
    }

    let loadstring = get_loadstring(input_code);

    let bytecode = get_bytecode(loadstring);

    let mut code = String::new();

    for byte in bytecode.split("\\") {
        if byte != "" {
            code.push(byte.parse::<u8>().expect("Failed to cast number") as char);
        }
    }

    println!("{}", code);
}
