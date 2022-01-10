

fn dfs(seen: &mut Vec<Vec<i32>>, data: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    if seen[i][j] == 1 { return 0 }
    seen[i][j] = 1;
    if data[i][j] == 9 { return 0 }
    let mut res = 1;
    if i>0 { res += dfs(seen, data, i-1, j);}
    if j>0 { res += dfs(seen, data, i, j-1);}
    if i<data.len()-1 {res += dfs(seen, data, i+1, j);}
    if j<data[i].len()-1 { res += dfs(seen, data, i, j+1);}

    res
}

fn main() {
    let lines = include_str!("input.txt").split('\n').collect::<Vec<_>>();
    let data = lines
        .iter()
        .map(|&line| line
            .chars()
            .map(|c| c
                .to_digit(10)
                .unwrap()
            )
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let n = data.len();
    let m = data[0].len();
    let mut seen = vec![vec![0;m];n];
    let mut basins = vec![];

    for i in 0..n {
        for j in 0..m {
            let basin = dfs(&mut seen, &data, i, j);
            if basin > 0 { basins.push(basin); println!("new basin at {}:{} - {}", i, j, basin);}
        }
    }

    basins.sort();
    println!("{:?}", basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap());
}
