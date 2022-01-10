

fn process_board(board: &mut Vec<i32>, num: i32) {
    for n in board.iter_mut() {
        if *n == num {
            *n = -1;
        }
    }
}

fn is_win(board: &Vec<i32>) -> bool {

    for chunk in board.chunks(5) {
        if chunk.iter().sum::<i32>() == -5 { return true }
    }
    for i in 0..5 {
        if board.iter().skip(i).step_by(5).sum::<i32>() == -5 { return true }
    }
    false
}

fn main() {
    let mut lines = include_str!("input.txt").split('\n').filter(|x| !x.is_empty()).collect::<Vec<_>>();
    let numbers = lines.remove(0).split(',').map(|x|x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut boards: Vec<Vec<i32>> = Vec::new();
    for chunk in lines.chunks(5) {
        let board = chunk
            .iter()
            .map(|line| line
                .split_whitespace()
                .map(|x|x
                    .parse::<i32>()
                    .unwrap()
                )
                .collect::<Vec<_>>()
            )
            .flatten()
            .collect::<Vec<_>>();
        println!("{:?}", board);
        boards.push(board);
    }
    println!("\n{:?}", numbers);

    let mut won = vec![0;100];
    for num in numbers {
        for i in 0..100 {
            if won[i] == 1 { continue; }
            let board = &mut boards[i];
            process_board(board, num);
            // println!("{:?}", board);
            if is_win(board) {
                won[i] = 1;
                if won.iter().sum::<i32>() == 100 {
                    println!("FOUND: {:?}", board);
                    let sum = board.iter().filter(|&x| x > &0).sum::<i32>();
                    println!("Res: {}", sum * num);
                    return;
                }
            }
        }
    }

}
