use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut depth = 0;
    let mut dist = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let v = l.split_whitespace().collect::<Vec<&str>>();
        let cmd = v[0];
        let arg: i32 = v[1].parse().unwrap();
        println!("{}:{}", cmd, arg);

        if cmd == "down" { aim = aim + arg; }
        if cmd == "up" { aim = aim - arg; }
        if cmd == "forward" { depth = depth + aim * arg; dist = dist + arg; }
    }
    println!("Depth: {}, Dist: {}, Res: {}", depth, dist, depth * dist);

}