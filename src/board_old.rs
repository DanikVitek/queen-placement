use anyhow::bail;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone)]
pub struct Board {
    pub width: u32,
    pub height: u32,
    pub pieces: Vec<PiecePlace>,
}

impl Board {
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pieces: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct PiecePlace(pub [u32; 2]);

impl From<[u32; 2]> for PiecePlace {
    fn from(val: [u32; 2]) -> Self {
        Self(val)
    }
}

impl From<(u32, u32)> for PiecePlace {
    fn from((x, y): (u32, u32)) -> Self {
        Self([x, y])
    }
}

impl Board {
    pub fn try_add_piece(&mut self, piece @ PiecePlace([x, y]): PiecePlace) -> anyhow::Result<()> {
        if x >= self.width || y >= self.height {
            bail!("Piece is out of the board")
        }

        if self
            .pieces
            .par_iter()
            .any(|PiecePlace([x1, y1])| &x == x1 && &y == y1)
        {
            bail!("Piece already exists");
        }

        self.pieces.push(piece);
        Ok(())
    }

    pub fn has_beaten(&self) -> bool {
        self.pieces.par_iter().any(|PiecePlace([x, y])| {
            self.pieces.par_iter().any(|PiecePlace([x1, y1])| {
                !(x == x1 && y == y1 || x != x1 && y != y1 && x.abs_diff(*x1) != y.abs_diff(*y1))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_beaten_test_horizontal() {
        let mut board = Board::new(5, 5);
        board.try_add_piece(PiecePlace([0, 0])).unwrap();
        board.try_add_piece(PiecePlace([1, 0])).unwrap();
        assert!(board.has_beaten())
    }

    #[test]
    fn has_beaten_test_vertical() {
        let mut board = Board::new(5, 5);
        board.try_add_piece(PiecePlace([0, 0])).unwrap();
        board.try_add_piece(PiecePlace([0, 1])).unwrap();
        assert!(board.has_beaten())
    }

    #[test]
    fn has_beaten_test_diagonal1() {
        let mut board = Board::new(5, 5);
        board.try_add_piece(PiecePlace([0, 0])).unwrap();
        board.try_add_piece(PiecePlace([1, 1])).unwrap();
        assert!(board.has_beaten())
    }

    #[test]
    fn has_beaten_test_diagonal2() {
        let mut board = Board::new(5, 5);
        board.try_add_piece(PiecePlace([1, 0])).unwrap();
        board.try_add_piece(PiecePlace([0, 1])).unwrap();
        assert!(board.has_beaten())
    }

    #[test]
    fn has_beaten_test_diagonal3() {
        let mut board = Board::new(5, 5);
        board.try_add_piece(PiecePlace([2, 0])).unwrap();
        board.try_add_piece(PiecePlace([1, 1])).unwrap();
        assert!(board.has_beaten())
    }

    #[test]
    fn has_beaten_test_diagonal4() {
        let mut board = Board::new(5, 5);
        board.try_add_piece(PiecePlace([1, 0])).unwrap();
        board.try_add_piece(PiecePlace([2, 1])).unwrap();
        assert!(board.has_beaten())
    }

    #[test]
    fn has_no_beaten_test1() {
        let mut board = Board::new(5, 5);
        board.try_add_piece(PiecePlace([0, 0])).unwrap();
        board.try_add_piece(PiecePlace([1, 2])).unwrap();
        assert!(!board.has_beaten())
    }

    #[test]
    fn has_no_beaten_test2() {
        let mut board = Board::new(5, 5);
        board.try_add_piece(PiecePlace([2, 0])).unwrap();
        board.try_add_piece(PiecePlace([0, 1])).unwrap();
        assert!(!board.has_beaten())
    }

    #[test]
    fn has_no_beaten_test3() {
        let mut board = Board::new(5, 5);
        board.try_add_piece(PiecePlace([2, 0])).unwrap();
        board.try_add_piece(PiecePlace([4, 1])).unwrap();
        assert!(!board.has_beaten())
    }
}
