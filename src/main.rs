use regex::Regex;
use std::{env, fs};

fn get_regex(input: String, regex: &str) -> String {
    let regex = Regex::new(regex).expect("Failed to compile regex");
    let mut output = String::new();

    for capture in regex.find(&input) {
        for i in capture.start()..capture.end() {
            output.push(
                input
                    .chars()
                    .nth(i)
                    .expect(&format!("Failed to get index {} in regex", i)),
            );
        }
        break;
    }

    return output;
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

    let loadstring = get_regex(input_code, r#"load(string)?\(("|'|\[\[).+("|'|\]\])\)\(\)"#);

    let bytecode = get_regex(loadstring, r#"(\\\d+)+"#);

    let mut code = String::new();

    for byte in bytecode.split("\\") {
        if byte != "" {
            code.push(byte.parse::<u8>().expect("Failed to cast number") as char);
        }
    }

    println!("{}", code);
}
