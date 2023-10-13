pub fn run_bytecode(bytecode: Vec<[u8; 2]>) {
    let mut index: usize = 0;
    let mut strip: Vec<u8> = Vec::new();
    strip.push(0);

    for token in bytecode {
        let action = token[0];
        let motion = get_motion(token[1], &index);
        if motion == 255 {
            continue; // Skip step if motion invalid
        }
        match char::from(action) {
            'O' => index = motion,
            '^' => {
                if strip.len() <= motion {
                    let mut pushes = motion - (strip.len() - 1);
                    while pushes > 0 {
                        strip.push(0);
                        pushes -= 1;
                    }
                }

                strip[motion] = strip[motion] + 1;
            }
            _ => {
                println!("invalid action: {}", char::from(action));
            }
        }
    }

    println!("Data:");
    for s in strip {
        println!("{}", s);
    }
}

fn get_motion(byte: u8, index: &usize) -> usize {
    match char::from(byte) {
        'U' => *index,
        'O' => index + 1,
        'Q' => index - 1,
        _ => {
            println!("invalid motion: {}", char::from(byte));
            255
        }
    }
}