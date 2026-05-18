use std::io::{Read, Write};

fn concat_file(input: &mut dyn Read, out: &mut dyn Write) -> i32 {
    let mut buffer: [u8; 4096] = [0; 4096];
    let mut count: i32 = 0;

    loop {
        let r = input.read(&mut buffer);
        let size = match r {
            Ok(size) => {
                size
            }
            Err(e) => {
                eprintln!("read error: {}", e);
                break;
            }
        };
        if size == 0 {
            break;
        }

        count += size as i32;
        let wr = out.write(&buffer[0..size]);
        if let Err(e) = wr {
            eprintln!("write error: {}", e);
            break;
        }
    }

    count
}

fn main() {
    let mut output = std::io::stdout();
    let files = std::env::args().skip(1);

    if files.len() == 0 {
        let mut input = std::io::stdin();
        concat_file(&mut input, &mut output);
        
    } else {
        for file in files {
            let fd = std::fs::File::open(&file);
            if let Err(e) = fd {
                eprintln!("open file '{}' error: {}", file, e);
                continue;
            }

            let mut input = fd.unwrap();
            concat_file(&mut input, &mut output);
        }
    }
}
