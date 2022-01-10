use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut res = -1;
    let mut last = -1;
    let mut depths: Vec<i32> = Vec::new();

    for line in reader.lines() {
        depths.push(line.unwrap().parse().unwrap());
    }
    for lines in depths.windows(3) {
        let sum = lines.iter().sum();
        if sum > last { res = res + 1 }
        last = sum;
        println!("window {:?} {}", lines, res)
    }
}