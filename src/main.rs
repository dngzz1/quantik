use std::io::Write;
use grid::Grid;

mod grid;

fn main() {
    let mut grid = Grid::new();
    let mut winner = None;
    for round in 0..16 {
        if grid.is_stuck(round) {
            break;
        }
        play_round(&mut grid, round);
        winner = grid.get_winner();
        if let Some(_) = winner {
            break;
        }
    }
    println!("{}", grid);
    if let Some(player) = winner {
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
    let pieces = grid.get_player_pieces(player);
    println!("Player {} has {:?}.", player, pieces);
    loop {
        let c = get_char_input("Please select a piece".to_owned());
        if !grid.player_has_piece(c, player) {
            println!("Player {} does not have {}", player, c);
            continue;
        }
        let pos = get_integer_input("Choose a position".to_owned());
        let result = grid.try_add(c, pos);
        if result.is_ok() {
            grid.try_remove(c, player).expect("coding error");
            break;
        } else {
            println!("{}", result.err().expect("error"));
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