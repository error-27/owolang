pub fn run_bytecode(bytecode: Vec<[u8; 2]>) {
    let mut index: usize = 0;
    let mut strip: Vec<u8> = Vec::new();
    strip.push(0);

    let mut str_count: u8 = 0;
    let mut stringinator: String = String::new();

    for token in bytecode {
        let action = token[0];
        let motion = get_motion(token[1], &index);
        if motion == 255 {
            continue; // Skip step if motion invalid
        }
        expand_strip(&mut strip, &motion);

        if str_count > 0 {
            for _ in 0..strip[motion] {
                stringinator = format!("{}{}", stringinator, char::from(action));
            }
            str_count -= 1;
            continue;
        }

        match char::from(action) {
            'O' => index = motion,
            '^' => {
                expand_strip(&mut strip, &motion);

                strip[motion] = strip[motion] + 1;
            },
            '-' => {
                expand_strip(&mut strip, &motion);

                strip[motion] = strip[motion] - 1;
            },
            'U' => {
                expand_strip(&mut strip, &motion);
                
                stringinator = String::new();
                str_count = strip[motion];
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
    println!("{}", stringinator);
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

fn expand_strip(strip: &mut Vec<u8>, to: &usize) {
    if strip.len() <= *to {
        let mut pushes = to - (strip.len() - 1);
        while pushes > 0 {
            strip.push(0);
            pushes -= 1;
        }
    }
}