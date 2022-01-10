
fn main() {
    let data = include_str!("input.txt").split(',').map(|x| x.chars().nth(0).unwrap().to_digit(10).unwrap()).collect::<Vec<_>>();
    println!("{:?}", data);

    let mut ages: Vec<i64> = vec![0;9];
    for i in data {
        ages[i as usize] += 1;
    }

    for day in 0..256 {
        let newfishes = ages[0];
        for i in 1..ages.len() {
            ages[i-1] = ages[i];
        }
        ages[6] += newfishes;
        ages[8] = newfishes;
        println!("Day {}: {} fishes", day, ages.iter().sum::<i64>());
    }
}
