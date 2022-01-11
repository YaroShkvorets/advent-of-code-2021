

fn main() {
    let lines = include_str!("input.txt").split('\n').collect::<Vec<_>>();
    let mut dots = vec![];
    let mut folds = vec![];
    for line in lines.iter() {
        if line.is_empty() { continue; }
        if line.starts_with("fold") {
            let parts = line.split('=').collect::<Vec<_>>();
            if parts[0].starts_with("fold along x") { folds.push(parts[1].parse::<i32>().unwrap()); }
            else { folds.push(-parts[1].parse::<i32>().unwrap())}
        }
        else {
            let dot = line.split(',').map(|x|x.parse::<i32>().unwrap()).collect::<Vec<_>>();
            dots.push(dot);
        }
    }

    for fold in &folds {
        let mut new_dots = vec![];
        if *fold < 0 {
            for dot in &dots {
                if dot[1] < -*fold { new_dots.push(dot.clone()); }
                else { new_dots.push(vec![dot[0], -fold - (dot[1]+fold)]); }
            }
        }
        else {
            for dot in &dots {
                if dot[0] < *fold { new_dots.push(dot.clone()); }
                else { new_dots.push(vec![fold - (dot[0]-fold), dot[1]]); }
            }
        }
        dots = new_dots;
        dots.sort();
        dots.dedup();
        println!("dots len: {}", dots.len());
    }

    let mut paper = vec![vec![' ';40];10];
    for dot in dots {
        paper[dot[1] as usize][dot[0] as usize] = '#';
    }
    for i in 0..paper.len() {
        for j in 0..paper[i].len() {
            print!("{}",paper[i][j]);
        }
        println!("");
    }

}