use std::fmt::{Display, Formatter};
use grid::Grid;

mod grid;

fn main() {
    // let mut player_0_pieces = vec!["A","A","B","B","C","C","D","D"];
    // let mut player_1_pieces = vec!["a","a","b","b","c","c","d","d"];
    let mut grid = Grid::new();
    grid.try_add('a', 2);
    grid.try_add('A', 3);
    println!("{}", grid);
}
