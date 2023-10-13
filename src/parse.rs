use std::io::{Result, Lines, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use std::process::exit;

pub fn prepare_file<P>(filename: P) -> Vec<[u8; 2]>
    where P: AsRef<Path>, {
    
    let lines = read_lines(filename).unwrap();
    let mut parsed: Vec<[u8; 2]> = Vec::new();

    for line in lines {
        if let Ok(l) = line {
            let mut converted = convert_owos(l);
            parsed.append(&mut converted);
        }
    }

    parsed
}

fn convert_owos(input: String) -> Vec<[u8; 2]> {
    let mut converted: Vec<[u8; 2]> = Vec::new();
    
    let tokens = input.trim().split(" ");
    for token in tokens {
        // Enforce OwO notation
        if token.len() != 3 || token.chars().nth(1).unwrap() != 'w' {
            println!("malformed owo: {}. skipping to next owo...", token);
            continue;
        }
        let no_w = token.replace("w", "");
        let byte_pair = no_w.as_bytes();
        converted.push(byte_pair.try_into().expect("owo improperly trimmed or malformed"));
    }

    converted
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}