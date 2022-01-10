
#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Vent {
    from: Point,
    to: Point
}

use std::cmp::max;

fn main() {
    let lines = include_str!("input.txt").split('\n').filter(|x| !x.is_empty()).collect::<Vec<_>>();
    let mut max_point = Point{x:0,y:0};
    let vents: Vec<Vent> = lines
        .iter()
        .map(|line| line
            .split_whitespace()
            .enumerate()
            .fold(Vent{from: Point{x:0,y:0}, to: Point{x:0,y:0}}, |mut acc, cur| {
                if cur.0 != 0 && cur.0 != 2 { return acc }
                let p = cur.1.split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
                if cur.0 == 0 { acc.from = Point{x:p[0],y:p[1]} }
                if cur.0 == 2 { acc.to = Point{x:p[0],y:p[1]} }
                if max_point.x < p[0] { max_point.x = p[0] }
                if max_point.y < p[1] { max_point.y = p[1] }
                acc
            })
        )
        .collect::<Vec<_>>();
    println!("Parsed vents: {}, max point: {:?}", vents.len(), max_point);

    let size = max(max_point.x, max_point.y) as usize;
    let mut board = vec![0; size*size];
    for vent in vents.iter() {
        let mut dx = if vent.from.x < vent.to.x { 1 } else { -1 };
        let mut dy = if vent.from.y < vent.to.y { 1 } else { -1 };
        if vent.from.x == vent.to.x { dx = 0 }
        if vent.from.y == vent.to.y { dy = 0 }
        let mut x = vent.from.x as i32;
        let mut y = vent.from.y as i32;
        while x != vent.to.x as i32 || y != vent.to.y as i32 {
            board[size * y as usize + x as usize] += 1;
            x += dx;
            y += dy;
            //println!("x: {} y: {}", x, y);
        }
        board[size * vent.to.y as usize + vent.to.x as usize] += 1;
    }
    for line in board.chunks(size) {
        for c in line.iter() {
            print!("{} ", if *c == 0 { String::from(".") } else { (*c).to_string() });
        }
        println!("\n");
    }
    let res = board.iter().filter(|&x|*x > 1).count();
    println!("Res: {}", res);
}
