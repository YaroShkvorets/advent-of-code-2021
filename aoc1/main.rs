use std::fs::File;
use std::io::{BufRead, BufReader};

// 1378
fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut res = 0;
    let mut depths: Vec<i32> = vec!();

    for line in reader.lines() {
        depths.push(line.unwrap().parse().unwrap());
    }
    for i in 3..depths.len() {
        if depths[i] > depths[i-3] { res = res + 1 }
        println!("window {} {}", i, res)
    }
}