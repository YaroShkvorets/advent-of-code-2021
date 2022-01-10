
fn main() {
    let lines = include_str!("input.txt").split('\n').collect::<Vec<_>>();

    let mut scores: Vec<i64> = vec![];
    for line in lines {
        let mut stack = vec![];
        let mut bad = false;
        for c in line.chars() {
            // print!("{}", c);
            if c=='{' || c=='[' || c=='(' || c=='<' { stack.push(c); continue;}
            if c==')' { if stack.pop().unwrap() != '(' { bad = true; break }}
            if c=='}' { if stack.pop().unwrap() != '{' { bad = true; break }}
            if c==']' { if stack.pop().unwrap() != '[' { bad = true; break }}
            if c=='>' { if stack.pop().unwrap() != '<' { bad = true; break }}
        }
        // println!("Good line {:?}", stack);
        if bad { continue; }
        let mut res = 0;
        for c in stack.iter().rev() {
            // println!("{}", res);
            if *c=='(' { res = res*5 + 1}
            if *c=='{' { res = res*5 + 3}
            if *c=='[' { res = res*5 + 2}
            if *c=='<' { res = res*5 + 4}
        }
        println!("{}", res);
        scores.push(res);
    }
    scores.sort();
    println!("Scores: {}, Res: {}", scores.len(),  scores[scores.len() / 2]);
}

// ((((<{<{{
