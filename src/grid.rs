use std::fmt::{Display, Formatter};

pub struct Grid {
    data: [Option<char>;16]
}

impl Grid {
    const REGIONS: [[usize;4];12] = [
        [1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16],
        [1,5,9,13],[2,6,10,14],[3,7,11,15],[4,8,12,16],
        [1,2,5,6],[3,4,7,8],[9,10,13,14],[11,12,15,16],
    ];

    pub fn new() -> Self {
        let data = [None;16];
        Self{data}
    }
    fn get(&self, pos: usize) -> Option<char> {
        return match self.data.get(pos - 1) {
            Some(&x) => x,
            None => None,
        }
    }

    fn can_place(&self, c: char, pos: usize) -> bool {
        if let Some(_) = self.get(pos) {return false;}
        get_all_regions(pos).into_iter()
            .map(|r| [self.get(r[0]),self.get(r[1]),self.get(r[2])])
            .all(|r| no_clash(&r,c))

    }

    pub fn try_add(&mut self, c: char, pos: usize) -> Result<(),String>{
        return match pos {
            1..=16 => {
                if self.can_place(c, pos) {
                    self.data[pos - 1] = Some(c);
                    return Ok(());
                }
                Err(format!("Invalid placement"))
            },
            i => Err(format!("Position {} is invalid", i))
        }
    }

}


impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pad_char = |c:Option<char>| match c {
            Some(x) => Some(format!(" {}", x)),
            None => None,
        };
        let d = self.data;
        let d1 = pad_char(d[0]).unwrap_or(" 1".to_owned());
        let d2 = pad_char(d[1]).unwrap_or(" 2".to_owned());
        let d3 = pad_char(d[2]).unwrap_or(" 3".to_owned());
        let d4 = pad_char(d[3]).unwrap_or(" 4".to_owned());
        let d5 = pad_char(d[4]).unwrap_or(" 5".to_owned());
        let d6 = pad_char(d[5]).unwrap_or(" 6".to_owned());
        let d7 = pad_char(d[6]).unwrap_or(" 7".to_owned());
        let d8 = pad_char(d[7]).unwrap_or(" 8".to_owned());
        let d9 = pad_char(d[8]).unwrap_or(" 9".to_owned());
        let d10 = pad_char(d[9]).unwrap_or("10".to_owned());
        let d11 = pad_char(d[10]).unwrap_or("11".to_owned());
        let d12 = pad_char(d[11]).unwrap_or("12".to_owned());
        let d13 = pad_char(d[12]).unwrap_or("13".to_owned());
        let d14 = pad_char(d[13]).unwrap_or("14".to_owned());
        let d15 = pad_char(d[14]).unwrap_or("15".to_owned());
        let d16 = pad_char(d[15]).unwrap_or("16".to_owned());
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
    for &r in Grid::REGIONS.iter() {
        if r[0] == pos {result.push([r[1],r[2],r[3]]);}
        if r[1] == pos {result.push([r[0],r[2],r[3]]);}
        if r[2] == pos {result.push([r[0],r[1],r[3]]);}
        if r[3] == pos {result.push([r[0],r[1],r[2]]);}
    }
    result
}

fn no_clash(arr: &[Option<char>;3], c: char) -> bool {
    let mut opponent_pieces;
    if c.is_uppercase() {
        opponent_pieces = arr.into_iter()
            .filter_map(|c| *c)
            .filter(|c| c.is_lowercase())
            .collect::<Vec<_>>();
    } else {
        opponent_pieces = arr.into_iter()
            .filter_map(|c| *c)
            .filter(|c| c.is_uppercase())
            .collect::<Vec<_>>();
    }
    !opponent_pieces.iter()
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .contains(&c)
}

#[cfg(test)]
mod tests {
    use crate::grid::{Grid, no_clash};

    #[test]
    fn no_clash_if_empty() {
        assert!(no_clash(&[None,None,None], 'a'));

    }
    #[test]
    fn no_clash_1() {
        assert!(no_clash(&[Some('A'),Some('B'),Some('d')], 'd'));
    }
    #[test]
    fn cannot_place() {
        assert!(!no_clash(&[Some('A'),None,None], 'a'));
    }
    #[test]
    fn can_place_in_empty_grid() {
        let grid = Grid::new();
        assert!(grid.can_place('a',1));
    }
    #[test]
    fn can_or_cannot_place_when_1_taken() {
        let mut grid = Grid::new();
        grid.try_add('A', 1).expect("Grid empty so should be ok");
        assert_eq!(grid.get(1), Some('A'));
        assert!(!grid.can_place('a', 1));
        assert!(!grid.can_place('a', 2));
        assert!(!grid.can_place('a', 3));
        assert!(!grid.can_place('a', 4));
        assert!(!grid.can_place('a', 5));
        assert!(!grid.can_place('a', 9));
        assert!(!grid.can_place('a', 13));
        assert!(!grid.can_place('a', 6));
        assert!(grid.can_place('a',7));
        assert!(grid.can_place('a',8));
        assert!(grid.can_place('a',10));
        assert!(grid.can_place('a',11));
        assert!(grid.can_place('a',12));
        assert!(grid.can_place('a',15));
        assert!(grid.can_place('a',16));
        for i in 2..=16 {
            assert!(grid.can_place('b', i));
        }

    }
}
