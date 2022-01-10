
fn main() {
    let lines = include_str!("input.txt").split('\n').collect::<Vec<_>>();

    let mut res = 0;
    for line in lines {
        let subl = line.split('|').collect::<Vec<_>>();
        // let nums = subl[0].split_whitespace().collect::<Vec<_>>();
        let data = subl[1].split_whitespace().collect::<Vec<_>>();
        for d in data {
            if d.len() == 2 || d.len() == 3 || d.len() == 4 || d.len() == 7 { res += 1;}
        }

    }
    println!("{:?}", res);

}

// a = 8 *from diff of 4 from 1
// f = 9 *the only that shows up 9 times
// c = 8 *left in 1
// b = 6 *the only one that shows up 6 times
// e = 4 *the only one that shows up 4 times
// d = 7 *left in 4
// g = 7 *last one