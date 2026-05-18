use std::io::{self, BufRead};

fn main() {
    let mut sum: i32 = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if line.is_err(){
            continue;
        }

        let content = line.unwrap(); 
        let num_result = content.parse::<i32>();
        if let Ok(num) = num_result {
            sum += num;
        } else {
            eprintln!("invalid format number: {}", content);
        }
    }

    println!("sum={}", sum);
}
