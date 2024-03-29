pub fn run_bytecode(bytecode: Vec<[u8; 2]>) {
    let mut index: usize = 0;
    let mut strip: Vec<u8> = Vec::new();
    strip.push(0);

    let mut str_count: u8 = 0;
    let mut stringinator: String = String::new();

    let mut i: usize = 0;

    while i < bytecode.len() {
        let token = bytecode[i];
        let action = token[0];
        let motion = get_motion(token[1], &index, &strip);
        if motion == 255 {
            continue; // Skip step if motion invalid
        }
        expand_strip(&mut strip, &motion);

        if str_count > 0 {
            for _ in 0..strip[motion] {
                stringinator = format!("{}{}", stringinator, char::from(action));
            }
            str_count -= 1;
            i += 1;
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
            },
            'V' => {
                expand_strip(&mut strip, &motion);
                for _ in 0..strip[motion] {
                    println!("{}", stringinator);
                }
            },
            'T' => {
                expand_strip(&mut strip, &motion);
                
                if strip[motion] == 0 {
                    i += 1;
                    continue;
                } else {
                    i += 2;
                    continue;
                }
            },
            'Q' => {
                expand_strip(&mut strip, &motion);

                if strip[motion] != 0 {
                    i += 1;
                    continue;
                } else {
                    i += 2;
                    continue;
                }
            },
            '=' => {
                expand_strip(&mut strip, &motion);

                i += strip[motion] as usize;
                continue;
            },
            '~' => {
                expand_strip(&mut strip, &motion);

                i -= strip[motion] as usize;
                continue;
            }
            _ => {
                println!("invalid action: {}", char::from(action));
            }
        }
        i += 1;
    }

    //println!("Data:");
    //for s in 0..strip.len() {
    //    println!("{}: {}", s, strip[s]);
    //}
    //println!("{}", stringinator);
}

fn get_motion(byte: u8, index: &usize, strip: &Vec<u8>) -> usize {
    match char::from(byte) {
        'U' => *index,
        'O' => index + 1,
        'Q' => index - 1,
        'p' => index - strip[*index] as usize,
        'e' => index + strip[*index] as usize,
        'T' => strip[*index] as usize,
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