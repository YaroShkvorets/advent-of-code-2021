
use std::cmp::max;

fn main() {
    let line = include_str!("input.txt");
    let data = line
        .split('=')
        .nth(2)
        .unwrap()
        .split("..")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let (bot, top) = (data[0], data[1]);

    let mut speed = 1;
    let mut res = 0;
    loop {
        speed += 1;
        let mut y = 0;
        let mut height = 0;
        let mut hit = false;
        let mut cur_speed = speed;
        loop {
            y += cur_speed;
            cur_speed -= 1;
            height = max(y, height);
            if y < bot { break; }
            if y >= bot && y <= top { res = height; hit = true; break; }
        }
        println!("Speed: {}, Reached: {}, hit: {}", speed, height, hit);
        if height > 20000 { break; }
    }

    println!("top: {}, bot: {}, res: {}", top, bot, res);
}