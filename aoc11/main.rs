
fn main() {
    let lines = include_str!("input.txt").split('\n').collect::<Vec<_>>();
    let mut data = lines
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

    let mut res = 0;
    for step in 0..1000 {
        for i in 0..data.len() {
            for j in 0..data.len() {
                if data[i][j] >= 500 { data[i][j] = 0;}
                data[i][j] += 1;
            }
        }
        let mut step_flashes = 0;
        loop {
            let flashes = flash(&mut data);
            println!("Flahes: {}", flashes);
            if flashes == 0 { break; }
            step_flashes += flashes;
        }
        println!("{}: {}", step, step_flashes);
        res += step_flashes;
        if step_flashes == data.len() as i32 * data[0].len() as i32 {
            println!("Found! {}", step);
            break;
        }
    }
    println!("{:?}", res);

}

fn flash(data: &mut Vec<Vec<u32>>) -> i32 {
    let mut res  = 0;
    for i in 0..data.len() {
        for j in 0..data.len() {
            if data[i][j] > 9 && data[i][j] < 500 {
                res += 1;
                if i>0 && j>0 {data[i-1][j-1] += 1;}
                if i>0 { data[i-1][j] += 1;}
                if j>0 { data[i][j-1] += 1;}
                if i<data.len()-1 { data[i+1][j] += 1;}
                if j<data[i].len()-1 {data[i][j+1] += 1;}
                if i<data.len()-1 && j<data[i].len()-1 { data[i+1][j+1] += 1;}
                if i>0 && j<data[i].len()-1 { data[i-1][j+1] += 1;}
                if j>0 && i<data.len()-1 { data[i+1][j-1] += 1;}
                data[i][j] = 500;
            }
        }
    }
    res
}

// ((((<{<{{
