use std::io::{Read, Write};

fn concat_file(input: &mut dyn Read, out: &mut dyn Write) -> i32 {
    let mut buffer: [u8; 4096] = [0; 4096];
    let mut count: i32 = 0;

    loop {
        let r = input.read(&mut buffer);
        if r.is_err() {
            eprintln!("read error: {}", r.unwrap_err());
            break;
        }

        let size = r.unwrap();
        if size == 0 {
            break;
        }

        count += size as i32;
        let wr = out.write(&mut buffer[0..size]);
        if wr.is_err() {
            eprintln!("write error: {}", wr.unwrap_err());
            break;
        }
    }

    return count;
}

fn main() {
    let mut output = std::io::stdout();
    let files = std::env::args().skip(1);

    if files.len() <= 0 {
        let mut input = std::io::stdin();
        concat_file(&mut input, &mut output);
        
    } else {
        for file in files {
            let mut fd = std::fs::File::open(file).expect("open file failed");
            concat_file(&mut fd, &mut output);
        }
    }
}
