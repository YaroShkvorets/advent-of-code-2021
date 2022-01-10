
use std::collections::HashMap;
use std::collections::HashSet;

fn is_big(cave: &str) -> bool {
    return cave.to_uppercase() == cave;
}

fn dfs(cave: &str, graph: &HashMap<&str, Vec<&str>>, seen: &HashSet<String>, mut seen_twice: bool, mut path: String) -> u32 {
    // println!("Enter {}", cave);
    if cave == "end" { println!("{}", path + " end"); return 1;}
    if cave == "start" && seen.contains(cave) { return 0;}
    if graph.get(cave).is_none() { return 0; }
    path += &(" ".to_string() + cave);
    if seen.contains(cave) { if seen_twice { return 0; } else { path += "*"; seen_twice = true; } }
    let mut res = 0;
    let mut new_seen = seen.clone();
    if !is_big(cave) { new_seen.insert(cave.to_string()); }
    for next in graph.get(cave).unwrap() {
        res += dfs(next, graph, &new_seen, seen_twice, path.clone());
    }
    res
}

fn main() {
    let lines = include_str!("input.txt").split('\n').collect::<Vec<_>>();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let seen: HashSet<String> = HashSet::new();
    for line in lines.iter() {
        let parts = line.split('-').collect::<Vec<_>>();
        if !graph.contains_key(parts[0]){ graph.insert(parts[0], vec![]); }
        if !graph.contains_key(parts[1]){ graph.insert(parts[1], vec![]); }
        graph.get_mut(parts[0]).unwrap().push(parts[1]);
        graph.get_mut(parts[1]).unwrap().push(parts[0]);
    }
    let res = dfs("start", &graph, &seen, false, "".to_string());

    println!("{:?}", res);

}
