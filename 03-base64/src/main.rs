use std::io::{self, Read, Write};
use clap::Parser;

mod base64;

/// Base64 encoder and decoder
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encode to base64 string
    #[arg(short, long, default_value_t = true)]
    encode: bool,

    /// Decode from base64 string
    #[arg(short, long)]
    decode: bool,

    /// Input file path, default is stdin
    #[arg(short, long, default_value = "-")]
    input: String,

    /// Output file path, default is stdout
    #[arg(short, long, default_value = "-")]
    output: String,
}

fn do_encode(input: &mut dyn Read, output: &mut dyn Write) {
    let buffer_size = 12 * 1024;
    let mut buffer: Vec<u8> = vec![0; buffer_size];

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

        let encoded = base64::base64_encode(&buffer[0..size]);
        let wr = output.write(&encoded);
        if wr.is_err() {
            eprintln!("write error: {}", wr.unwrap_err());
            break;
        }
    }

    _ = output.write(b"\n");
}

fn main() {
    let args = Args::parse();

    let input_filename = args.input;
    let mut input: Box<dyn Read> = if input_filename == "-" {
        Box::new(io::stdin())

    } else {
        let fd = std::fs::File::open(&input_filename);
        if fd.is_err() {
            eprintln!("open file '{}' error: {}", &input_filename, fd.unwrap_err());
            return;
        }
        Box::new(fd.unwrap())
    };

    let output_filename = args.output;
    let mut output: Box<dyn Write> = if output_filename == "-" {
        Box::new(io::stdout())

    } else {
        let fd = std::fs::File::create(&output_filename);
        if fd.is_err() {
            eprintln!("create file '{}' error: {}", &output_filename, fd.unwrap_err());
            return;
        }
        Box::new(fd.unwrap())
    };

    if args.encode {
        do_encode(&mut input, &mut output);

    } else {
        eprintln!("please specify encode or decode");
    }
}