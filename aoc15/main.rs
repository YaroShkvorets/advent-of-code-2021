

use std::collections::BinaryHeap;

fn get_risk(data: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let (n, m) = ( data.len(), data[0].len() );
    let dist = (i/n + j/m) as i32;
    let base = data[i%n][j%m] + dist ;
    if base <= 9 { base } else { base%9 }
}
fn main() {
    let lines = include_str!("input.txt").split('\n').collect::<Vec<_>>();
    let data = lines
        .iter()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let (n, m) = (data.len(), data[0].len());
    let mut pq = BinaryHeap::new();
    let mut seen = vec![vec![0;m*5];n*5];
    let mut res = 0;

    pq.push((0,0,0));
    while pq.len() > 0 {
        let (w,i,j) = pq.pop().unwrap();
        if seen[i][j] == 1 { continue; };
        seen[i][j] = 1;
        if i == 5*n-1 && j == 5*m-1 {
            res = -w;
            break;
        }
        if i>0 { pq.push((w - get_risk(&data, i-1, j), i-1, j)) }
        if j>0 { pq.push((w - get_risk(&data, i, j-1), i, j-1)) }
        if i<5*n-1 { pq.push((w - get_risk(&data, i+1, j), i+1, j)) }
        if j<5*m-1 { pq.push((w - get_risk(&data, i, j+1), i, j+1)) }
    }
    println!("\nres: {}", res);
}