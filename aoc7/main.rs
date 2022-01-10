

use std::cmp::min;

fn main() {
    let data = include_str!("input.txt").split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    println!("{:?}", data);

    let pos_min = data.iter().min().unwrap();
    let pos_max = data.iter().max().unwrap();

    let mut res = i32::MAX;
    for pos in *pos_min..=*pos_max {
        let mut burnt = 0;
        for i in data.iter() {
            let m = (i-pos).abs();
            burnt += m*(1 + m)/2;
        }
        res = min(res, burnt);
    }
    println!("Res: {}", res);
}
