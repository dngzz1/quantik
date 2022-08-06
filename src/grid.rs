use std::fmt::{Display, Formatter};
use itertools::Itertools;
#[derive(PartialEq, Eq)]
pub struct Player(pub i32);
struct Region([usize;4]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Piece(pub char);
pub struct Grid {
    /// A 4x4 grid.
    data: [Option<Piece>;16],
    pub player_0_pieces: Vec<Piece>,
    pub player_1_pieces: Vec<Piece>,
}

impl Grid {
    const REGIONS: [Region;12] = [
        Region([1,2,3,4]),Region([5,6,7,8]),Region([9,10,11,12]),Region([13,14,15,16]),
        Region([1,5,9,13]),Region([2,6,10,14]),Region([3,7,11,15]),Region([4,8,12,16]),
        Region([1,2,5,6]),Region([3,4,7,8]),Region([9,10,13,14]),Region([11,12,15,16]),
    ];

    pub fn new() -> Self {
        let data = [None;16];
        let player_0_pieces = vec![Piece('A'),Piece('A'),Piece('B'),Piece('B'),
                                   Piece('C'),Piece('C'),Piece('D'),Piece('D')];
        let player_1_pieces = vec![Piece('a'),Piece('a'),Piece('b'),Piece('b'),
                                   Piece('c'),Piece('c'),Piece('d'),Piece('d')];
        Self{data, player_0_pieces, player_1_pieces }
    }
    fn get(&self, pos: usize) -> Option<Piece> {
        return match self.data.get(pos - 1) {
            Some(&x) => x,
            None => None,
        }
    }

    fn can_place(&self, piece: &Piece, pos: usize) -> bool {
        if self.get(pos).is_some() {return false;}
        get_all_regions(pos).into_iter()
            .map(|r| [self.get(r[0]),self.get(r[1]),self.get(r[2])])
            .all(|r| no_clash(&r,piece))

    }

    pub fn try_add(&mut self, piece: Piece, pos: usize) -> Result<(),String>{
        return match pos {
            1..=16 => {
                if self.can_place(&piece, pos) {
                    self.data[pos - 1] = Some(piece);
                    return Ok(());
                }
                Err(format!("[Error: Invalid placement at {}]", pos))
            },
            i => Err(format!("[Error: Position {} is invalid]", i))
        }
    }

    pub fn player_has_piece(&self, piece: &Piece, player: Player) -> bool {
        if !vec![Player(0), Player(1)].contains(&player) {
            return false;
        }
        let pieces = if player == Player(0) {&self.player_0_pieces} else {&self.player_1_pieces};
        return pieces.iter().any(|x| x == piece)
    }

    pub fn try_remove(&mut self, piece: Piece, player: Player) -> Result<(),String> {
        let pieces = if player == Player(0) {&mut self.player_0_pieces} else {&mut self.player_1_pieces};
        if let Some(pos) = pieces.iter().position(|&x| x == piece) {
            pieces.remove(pos);
            Ok(())
        } else {
            let Piece(c) = piece;
            Err(format!("[Error: {} not found]", c))
        }
    }

    fn get_winner_from_region(&self, region: Region) -> Option<Player> {
        let Region(r) = region;
        let pieces = [self.get(r[0]), self.get(r[1]), self.get(r[2]), self.get(r[3])];
        if pieces.into_iter()
            .flatten()
            .filter(|Piece(x)| x.is_uppercase())
            .unique().count() == 4 {
            return Some(Player(0));
        } else if pieces.into_iter()
            .flatten()
            .filter(|Piece(x)| x.is_lowercase())
            .unique().count() == 4 {
            return Some(Player(1));
        }
        None
    }
    pub fn get_winner(&self) -> Option<Player> {
        for region in Grid::REGIONS {
            if let Some(player) = self.get_winner_from_region(region) {
                return Some(player);
            }
        }
        None
    }

    pub fn get_player_pieces(&self, player: Player) -> &Vec<Piece> {
        if player == Player(0) {
            &self.player_0_pieces
        } else {
            &self.player_1_pieces
        }
    }

    pub fn is_stuck(&self, player: Player) -> bool {
        let pieces = self.get_player_pieces(player);
        for piece in pieces {
            for pos in 1..=16 {
                if self.can_place(piece, pos) {
                    return false;
                }
            }
        }
        true
    }
}


impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pad_char = |c:Option<char>| c.map(|x| format!(" {}",x));
        let d = self.data.map(|x| x.map(|Piece(c)| c));
        let d1 = pad_char(d[0]).unwrap_or_else(|| " 1".to_owned());
        let d2 = pad_char(d[1]).unwrap_or_else(|| " 2".to_owned());
        let d3 = pad_char(d[2]).unwrap_or_else(|| " 3".to_owned());
        let d4 = pad_char(d[3]).unwrap_or_else(|| " 4".to_owned());
        let d5 = pad_char(d[4]).unwrap_or_else(|| " 5".to_owned());
        let d6 = pad_char(d[5]).unwrap_or_else(|| " 6".to_owned());
        let d7 = pad_char(d[6]).unwrap_or_else(|| " 7".to_owned());
        let d8 = pad_char(d[7]).unwrap_or_else(|| " 8".to_owned());
        let d9 = pad_char(d[8]).unwrap_or_else(|| " 9".to_owned());
        let d10 = pad_char(d[9]).unwrap_or_else(|| "10".to_owned());
        let d11 = pad_char(d[10]).unwrap_or_else(|| "11".to_owned());
        let d12 = pad_char(d[11]).unwrap_or_else(|| "12".to_owned());
        let d13 = pad_char(d[12]).unwrap_or_else(|| "13".to_owned());
        let d14 = pad_char(d[13]).unwrap_or_else(|| "14".to_owned());
        let d15 = pad_char(d[14]).unwrap_or_else(|| "15".to_owned());
        let d16 = pad_char(d[15]).unwrap_or_else(|| "16".to_owned());
        write!(f,"\
        ==================\n\
        {} | {} | {} | {}\n\
        ==================\n\
        {} | {} | {} | {}\n\
        ==================\n\
        {} | {} | {} | {}\n\
        ==================\n\
        {} | {} | {} | {}\n\
        ==================", d1,d2,d3,d4,d5,d6,d7,d8,d9,d10,d11,d12,d13,d14,d15,d16)
    }
}

fn get_all_regions(pos: usize) -> Vec<[usize;3]> {
    let mut result = vec![];
    for &Region(r) in Grid::REGIONS.iter() {
        if r[0] == pos {result.push([r[1],r[2],r[3]]);}
        if r[1] == pos {result.push([r[0],r[2],r[3]]);}
        if r[2] == pos {result.push([r[0],r[1],r[3]]);}
        if r[3] == pos {result.push([r[0],r[1],r[2]]);}
    }
    result
}

fn no_clash(arr: &[Option<Piece>;3], piece: &Piece) -> bool {
    let Piece(c) = piece;
    let opponent_pieces = if c.is_uppercase() {
        arr.iter()
            .filter_map(|piece| *piece)
            .filter(|Piece(c)| c.is_lowercase())
            .collect::<Vec<_>>()
    } else {
        arr.iter()
            .filter_map(|piece| *piece)
            .filter(|Piece(c)| c.is_uppercase())
            .collect::<Vec<_>>()
    };
    !opponent_pieces.iter()
        .map(|Piece(c)| c.to_ascii_lowercase())
        .any(|x| &x == c)
}

#[cfg(test)]
mod tests {
    use crate::grid::{Grid, no_clash, Piece};
    use crate::Player;

    #[test]
    fn no_clash_if_empty() {
        assert!(no_clash(&[None,None,None], &Piece('a')));

    }
    #[test]
    fn no_clash_1() {
        assert!(no_clash(&[Some(Piece('A')),Some(Piece('B')),Some(Piece('d'))], &Piece('d')));
    }
    #[test]
    fn cannot_place() {
        assert!(!no_clash(&[Some(Piece('A')),None,None], &Piece('a')));
    }
    #[test]
    fn can_place_in_empty_grid() {
        let grid = Grid::new();
        assert!(grid.can_place(&Piece('a'),1));
    }
    #[test]
    fn can_or_cannot_place_when_1_taken() {
        let mut grid = Grid::new();
        grid.try_add(Piece('A'), 1).expect("Grid empty so should be ok");
        assert_eq!(grid.get(1), Some(Piece('A')));
        assert!(!grid.can_place(&Piece('a'), 1));
        assert!(!grid.can_place(&Piece('a'), 2));
        assert!(!grid.can_place(&Piece('a'), 3));
        assert!(!grid.can_place(&Piece('a'), 4));
        assert!(!grid.can_place(&Piece('a'), 5));
        assert!(!grid.can_place(&Piece('a'), 9));
        assert!(!grid.can_place(&Piece('a'), 13));
        assert!(!grid.can_place(&Piece('a'), 6));
        assert!(grid.can_place(&Piece('a'),7));
        assert!(grid.can_place(&Piece('a'),8));
        assert!(grid.can_place(&Piece('a'),10));
        assert!(grid.can_place(&Piece('a'),11));
        assert!(grid.can_place(&Piece('a'),12));
        assert!(grid.can_place(&Piece('a'),15));
        assert!(grid.can_place(&Piece('a'),16));
        for i in 2..=16 {
            assert!(grid.can_place(&Piece('b'), i));
        }
    }
    #[test]
    fn can_remove_from_player_0() {
        let mut grid = Grid::new();
        assert_eq!(grid.player_0_pieces.len(), 8);
        grid.try_remove(Piece('A'),Player(0)).expect("Cannot remove");
        assert_eq!(grid.player_0_pieces.len(), 7);
    }
}
