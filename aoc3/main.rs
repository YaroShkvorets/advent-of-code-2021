
fn get_rating(mut arr: Vec<&str>, c1: char, c2: char) -> Result<u32, ()>
{
    let mut i = 0;
    while arr.len() > 1 && i < 12 {
        let ones = arr.iter().fold(0, |acc,cur| acc + cur.chars().nth(i).unwrap().to_digit(10).unwrap());
        let winner = if ones * 2 >= arr.len() as u32 { c1 } else { c2 };
        arr = arr.into_iter().filter(|&s| s.chars().nth(i).unwrap() == winner ).collect::<Vec<_>>();
        i += 1;
    }
    Ok(isize::from_str_radix(arr[0], 2).unwrap() as u32)
}

fn main() {
    let orig = include_str!("input.txt").split_whitespace().collect::<Vec<_>>();

    let oxyn = get_rating(orig.clone(), '1', '0').unwrap();
    let co2n = get_rating(orig.clone(), '0', '1').unwrap();

    println!("Orig len: {:?}, Oxy: {}, CO2: {}, res: {}", orig.len(), oxyn, co2n, oxyn * co2n);

}
