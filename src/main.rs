use std::io::Write;
use grid::Grid;
use crate::grid::{Piece, Player};

mod grid;

fn main() {
    let mut grid = Grid::new();
    let mut winner = None;
    let mut round = 0;
    while winner.is_none() {
        if grid.is_stuck(Player(round)) {
            break;
        }
        play_round(&mut grid, round);
        winner = grid.get_winner();
        if winner.is_some() {
            break;
        }
        round += 1;
    }
    println!("{}", grid);
    if let Some(Player(player)) = winner {
        println!("Player {} is the winner!", player);
    } else {
        println!("The game is a draw");
    }
    println!("======== THE END ========");
}

fn play_round(grid: &mut Grid, round: i32) {
    println!("Round {}:", round);
    println!("{}", &grid);
    let player = round % 2;
    let pieces = grid.get_player_pieces(Player(player));
    println!("Player {} has {:?}.", player, pieces);
    loop {
        let c = get_char_input(format!("Player {}, select a piece", player));
        if !grid.player_has_piece(&Piece(c), Player(player)) {
            println!("Player {} does not have {}", player, c);
            continue;
        }
        let pos = get_integer_input(format!("Player {}, choose a position", player));
        let result = grid.try_add(Piece(c), pos);
        if result.is_ok() {
            grid.try_remove(Piece(c), Player(player)).expect("coding error");
            break;
        } else {
            println!("{}", result.expect_err("coding error"));
        }
    }
}

fn get_char_input(message: String) -> char {
    loop {
        let mut input = String::new();
        print!("{}: ", &message);
        std::io::stdout().flush().expect("Failed to flush");
        std::io::stdin().read_line(&mut input).expect("Failed to read");
        match input.trim().parse::<char>() {
            Ok(a) => return a,
            Err(_) => { println!("[Error: not a char]"); continue; },
        }
    }
}

fn get_integer_input(message: String) -> usize {
    loop {
        let mut input = String::new();
        print!("{}: ", &message);
        std::io::stdout().flush().expect("Failed to flush");
        std::io::stdin().read_line(&mut input).expect("Failed to read");
        match input.trim().parse::<usize>() {
            Ok(int) => return int,
            Err(_) => { println!("[Error: not an integer]"); continue; },
        }
    }
}