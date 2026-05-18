use std::io::{Read};
use clap::{Parser};
use crate::checksum::Checksum;

mod checksum;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // checksum algorithm, default is POSIX CRC checksum
    #[arg(short, default_value="0")]
    option: i32,

    files: Vec<String>,
}

fn checksum_file(input: &mut dyn Read, algo: &mut dyn Checksum) -> (u32, usize) {
    let buffer_size = 12 * 1024;
    let mut buffer: Vec<u8> = vec![0; buffer_size];
    let mut length = 0;

    loop {
        let r = input.read(&mut buffer);
        let size = match r {
            Ok(size) => size,
            Err(e) => {
                eprintln!("read error: {}", e);
                break;
            }
        };
        if size == 0 {
            break;
        }

        length += size;
        algo.update(&buffer[0..size]);
    }

    let sum = algo.checksum();
    (sum, length)
}

fn main() {
    let args = Args::parse();
    if args.option < 0 || args.option > 3 {
        eprintln!("invalid option: {}", args.option);
        return;
    }

    let create_algo = match args.option {
        0 => checksum::PosixSum::build,
        1 => checksum::BSDSum::build,
        2 => checksum::ATTSum::build,
        3 => checksum::ISOSum::build,
        _ => checksum::PosixSum::build,
    };

    if !args.files.is_empty() {
        for file in args.files {
            let mut input = std::fs::File::open(&file).unwrap();
            let mut algo = create_algo();
            let (sum, length) = checksum_file(&mut input,  algo.as_mut());
            println!("{} {} {}", sum, length, file)
        }
    } else {
        let mut input = std::io::stdin();
        let mut algo = create_algo();
        let (sum, length) = checksum_file(&mut input,  algo.as_mut());
        println!("{} {}", sum, length)
    }
}
