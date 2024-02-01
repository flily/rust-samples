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
        if r.is_err() {
            eprintln!("read error: {}", r.unwrap_err());
            break;
        }

        let size = r.unwrap();
        if size <= 0 {
            break;
        }

        length += size;
        algo.update(&buffer[0..size]);
    }

    let sum = algo.checksum();
    return (sum, length);
}

fn main() {
    let args = Args::parse();
    if args.option < 0 || args.option > 3 {
        eprintln!("invalid option: {}", args.option);
        return;
    }

    let create_algo = match args.option {
        0 => checksum::create_posix_checksum,
        1 => checksum::create_bsd_checksum,
        2 => checksum::create_att_checksum,
        3 => checksum::create_iso_checksum,
        _ => checksum::create_posix_checksum,
    };

    if args.files.len()>0 {
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
