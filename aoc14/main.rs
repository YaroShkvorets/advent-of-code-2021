

use std::collections::HashMap;
use std::cmp::max;
use std::cmp::min;

//only part 1 implemented

fn main() {
    let lines = include_str!("input.txt").split('\n').collect::<Vec<_>>();
    let mut data = lines[0].chars().collect::<Vec<_>>();
    let mut rules = HashMap::new();
    lines
        .iter()
        .skip(2)
        .for_each(|s| {
            let r = s.split(" -> ").collect::<Vec<_>>();
            rules.insert(r[0], r[1].chars().nth(0).unwrap());
        });

    for step in 0..10 {
        let mut new_data = vec![];
        for c in data {
            if new_data.is_empty() { new_data.push(c); continue; }
            let s = &format!("{}{}",new_data.last().unwrap(),c)[..];
            if rules.contains_key(&s) {
                new_data.push(*rules.get(&s).unwrap());
            }
            new_data.push(c);
        }
        data = new_data;
        println!("Step: {}, len: {}", step, data.len());
    }
    let mut freq = HashMap::new();
    for c in &data {
        if !freq.contains_key(c) { freq.insert(c, 0); }
        *freq.get_mut(c).unwrap() += 1;
    }
    let mut n_max = 0;
    let mut n_min = i32::MAX;
    for p in freq {
        n_min = min(n_min, p.1);
        n_max = max(n_max, p.1);
    }

    println!("{:?}\nres: {}", data.iter().collect::<String>(), n_max - n_min);
}