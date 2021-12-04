use std::fs;
fn parse_input(contents: &String) -> (Vec<&str>, Vec<u32>) {
    let lines: Vec<&str> = contents.lines().collect();
    let mut selection: Vec<u32> = Vec::new();
    let mut boards = Vec::new();
    lines.iter().for_each(|line| {
        if line.len() > 14 {
            selection = line.split(",").map(|n| n.parse().unwrap()).collect();
        } else {
            boards.push(line.to_owned());
        }
    });
    (boards, selection)
}
fn parse_boards(boards: Vec<&str>) -> Vec<Vec<Vec<u32>>> {
    let mut boards_vec: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut temp: Vec<Vec<u32>> = Vec::new();
    boards.iter().for_each(|line| {
        if line.len() > 0 {
            temp.push(
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        }
        if temp.len() == 5 {
            boards_vec.push(temp.clone());
            temp = Vec::new();
        }
    });
    boards_vec
}

fn sum_selected(vertical: &mut Vec<u32>, horizontal: &mut Vec<u32>, board: Vec<&u32>) -> u32 {
    let selected = vertical;
    selected.append(horizontal);

    board
        .iter()
        .map(|num| num.to_owned())
        .filter(|num| !selected.contains(num))
        .sum()
}
fn bingo_first_winner(selection: &Vec<u32>, boards: Vec<Vec<Vec<u32>>>) -> u32 {
    let mut horizontal: Vec<Vec<Vec<u32>>> = vec![vec![Vec::new(); 5]; boards.len()];
    let mut vertical: Vec<Vec<Vec<u32>>> = vec![vec![Vec::new(); 5]; boards.len()];

    let mut winning_board_idx: usize = 0;
    let mut winner: Vec<u32> = Vec::new();
    let mut idx = 0;
    loop {
        if winner.len() > 0 {
            break;
        }

        for i in 0..boards.len() {
            for j in 0..5 {
                for k in 0..5 {
                    if boards[i][j][k] == selection[idx] {
                        horizontal[i][j].push(selection[idx]);
                        vertical[i][k].push(selection[idx]);
                        if horizontal[i][j].len() == 5 {
                            winning_board_idx = i;
                            winner = horizontal[i][j].to_vec();
                        }
                        if vertical[i][k].len() == 5 {
                            winning_board_idx = i;
                            winner = vertical[i][k].to_vec();
                        }
                    }
                }
            }
        }
        idx += 1;
    }

    let selected_sum = sum_selected(
        &mut vertical[winning_board_idx]
            .iter()
            .flatten()
            .map(|n| n.to_owned())
            .collect(),
        &mut horizontal[winning_board_idx]
            .iter()
            .flatten()
            .map(|n| n.to_owned())
            .collect(),
        boards[winning_board_idx].iter().flatten().collect(),
    );

    winner[winner.len() - 1] * selected_sum
}

fn bingo_last_winner(selection: &Vec<u32>, boards: Vec<Vec<Vec<u32>>>) -> u32 {
    let mut horizontal: Vec<Vec<Vec<u32>>> = vec![vec![Vec::new(); 5]; boards.len()];
    let mut vertical: Vec<Vec<Vec<u32>>> = vec![vec![Vec::new(); 5]; boards.len()];

    let mut winners: Vec<usize> = Vec::new();
    let mut last_bingo: Vec<u32> = Vec::new();
    let mut idx = 0;
    loop {
        if winners.len() == 100 {
            break;
        }
        for i in 0..boards.len() {
            for j in 0..5 {
                for k in 0..5 {
                    if boards[i][j][k] == selection[idx] {
                        horizontal[i][j].push(selection[idx]);
                        vertical[i][k].push(selection[idx]);

                        if horizontal[i][j].len() == 5 {
                            if !winners.contains(&i) {
                                winners.push(i);
                                last_bingo = horizontal[i][j].to_vec();
                            }
                        }
                        if vertical[i][k].len() == 5 {
                            if !winners.contains(&i) {
                                winners.push(i);
                                last_bingo = vertical[i][k].to_vec();
                            }
                        }
                    }
                }
            }
        }
        idx += 1;
    }

    let last_winner_idx = winners[winners.len() - 1];
    let selected_sum = sum_selected(
        &mut vertical[last_winner_idx]
            .iter()
            .flatten()
            .map(|n| n.to_owned())
            .collect(),
        &mut horizontal[last_winner_idx]
            .iter()
            .flatten()
            .map(|n| n.to_owned())
            .collect(),
        boards[last_winner_idx].iter().flatten().collect(),
    );

    last_bingo[last_bingo.len() - 1] * selected_sum
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!(
        "First winner: {:?}",
        bingo_first_winner(
            &parse_input(&contents).1,
            parse_boards(parse_input(&contents).0)
        )
    );
    println!(
        "Last winner: {:?}",
        bingo_last_winner(
            &parse_input(&contents).1,
            parse_boards(parse_input(&contents).0)
        )
    );
}
