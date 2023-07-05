use std::fmt::{Display, Error, Formatter};
use std::ops::{BitAnd, BitOr, BitXor, Not};

use crate::square::Square;

/// A bitboard.
///
/// Each bit corresponds to a square on the board, with the bit set to 1 indicating that a piece is present at that square.
///
/// This bitboard uses a vertical layout, meaning that each rank of the board (from 1 to 9) is represented by a 9-bit row.
/// Consequently, the whole board is represented by the first 81 bits of the 128-bit integer.
///
/// This design allows for efficient computation and evaluation of move legality, piece attacks, and other board states.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Bitboard(u128);

impl Bitboard {
    #[inline(always)]
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub fn is_any(&self) -> bool {
        self.0 != 0
    }
}

impl BitAnd for &Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for &Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor for &Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl Not for &Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for rank in 0..9 {
            for file in (0..9).rev() {
                let square = Square::from_coord(file, rank);
                if self.0 & (1 << square.index()) != 0 {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard((1 << 81) - 1);

    pub const FILE_1: Bitboard = Bitboard(0x1FF << 9 * 0);
    pub const FILE_2: Bitboard = Bitboard(0x1FF << 9 * 1);
    pub const FILE_3: Bitboard = Bitboard(0x1FF << 9 * 2);
    pub const FILE_4: Bitboard = Bitboard(0x1FF << 9 * 3);
    pub const FILE_5: Bitboard = Bitboard(0x1FF << 9 * 4);
    pub const FILE_6: Bitboard = Bitboard(0x1FF << 9 * 5);
    pub const FILE_7: Bitboard = Bitboard(0x1FF << 9 * 6);
    pub const FILE_8: Bitboard = Bitboard(0x1FF << 9 * 7);
    pub const FILE_9: Bitboard = Bitboard(0x1FF << 9 * 8);

    pub const RANK_1: Bitboard = Bitboard(0x1008040201008040201 << 0);
    pub const RANK_2: Bitboard = Bitboard(0x1008040201008040201 << 1);
    pub const RANK_3: Bitboard = Bitboard(0x1008040201008040201 << 2);
    pub const RANK_4: Bitboard = Bitboard(0x1008040201008040201 << 3);
    pub const RANK_5: Bitboard = Bitboard(0x1008040201008040201 << 4);
    pub const RANK_6: Bitboard = Bitboard(0x1008040201008040201 << 5);
    pub const RANK_7: Bitboard = Bitboard(0x1008040201008040201 << 6);
    pub const RANK_8: Bitboard = Bitboard(0x1008040201008040201 << 7);
    pub const RANK_9: Bitboard = Bitboard(0x1008040201008040201 << 8);

    pub const SQUARES: [Bitboard; 81] = [
        Bitboard(1 << 0),
        Bitboard(1 << 1),
        Bitboard(1 << 2),
        Bitboard(1 << 3),
        Bitboard(1 << 4),
        Bitboard(1 << 5),
        Bitboard(1 << 6),
        Bitboard(1 << 7),
        Bitboard(1 << 8),
        Bitboard(1 << 9),
        Bitboard(1 << 10),
        Bitboard(1 << 11),
        Bitboard(1 << 12),
        Bitboard(1 << 13),
        Bitboard(1 << 14),
        Bitboard(1 << 15),
        Bitboard(1 << 16),
        Bitboard(1 << 17),
        Bitboard(1 << 18),
        Bitboard(1 << 19),
        Bitboard(1 << 20),
        Bitboard(1 << 21),
        Bitboard(1 << 22),
        Bitboard(1 << 23),
        Bitboard(1 << 24),
        Bitboard(1 << 25),
        Bitboard(1 << 26),
        Bitboard(1 << 27),
        Bitboard(1 << 28),
        Bitboard(1 << 29),
        Bitboard(1 << 30),
        Bitboard(1 << 31),
        Bitboard(1 << 32),
        Bitboard(1 << 33),
        Bitboard(1 << 34),
        Bitboard(1 << 35),
        Bitboard(1 << 36),
        Bitboard(1 << 37),
        Bitboard(1 << 38),
        Bitboard(1 << 39),
        Bitboard(1 << 40),
        Bitboard(1 << 41),
        Bitboard(1 << 42),
        Bitboard(1 << 43),
        Bitboard(1 << 44),
        Bitboard(1 << 45),
        Bitboard(1 << 46),
        Bitboard(1 << 47),
        Bitboard(1 << 48),
        Bitboard(1 << 49),
        Bitboard(1 << 50),
        Bitboard(1 << 51),
        Bitboard(1 << 52),
        Bitboard(1 << 53),
        Bitboard(1 << 54),
        Bitboard(1 << 55),
        Bitboard(1 << 56),
        Bitboard(1 << 57),
        Bitboard(1 << 58),
        Bitboard(1 << 59),
        Bitboard(1 << 60),
        Bitboard(1 << 61),
        Bitboard(1 << 62),
        Bitboard(1 << 63),
        Bitboard(1 << 64),
        Bitboard(1 << 65),
        Bitboard(1 << 66),
        Bitboard(1 << 67),
        Bitboard(1 << 68),
        Bitboard(1 << 69),
        Bitboard(1 << 70),
        Bitboard(1 << 71),
        Bitboard(1 << 72),
        Bitboard(1 << 73),
        Bitboard(1 << 74),
        Bitboard(1 << 75),
        Bitboard(1 << 76),
        Bitboard(1 << 77),
        Bitboard(1 << 78),
        Bitboard(1 << 79),
        Bitboard(1 << 80),
    ];
}

impl From<Square> for &Bitboard {
    fn from(sq: Square) -> Self {
        &Bitboard::SQUARES[sq.index()]
    }
}

impl From<Square> for Bitboard {
    fn from(sq: Square) -> Self {
        Bitboard::SQUARES[sq.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::Square;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case(
        Bitboard::EMPTY,
        indoc! {"
            000000000
            000000000
            000000000
            000000000
            000000000
            000000000
            000000000
            000000000
            000000000
        "}
    )]
    #[case(
        Bitboard::FULL,
        indoc! {"
            111111111
            111111111
            111111111
            111111111
            111111111
            111111111
            111111111
            111111111
            111111111
        "}
    )]
    #[case(
        Bitboard::FILE_8,
        indoc! {"
            010000000
            010000000
            010000000
            010000000
            010000000
            010000000
            010000000
            010000000
            010000000
        "}
    )]
    #[case(
        Bitboard::RANK_3,
        indoc! {"
            000000000
            000000000
            111111111
            000000000
            000000000
            000000000
            000000000
            000000000
            000000000
        "}
    )]
    #[case(
        Square::SQ_11.into(),
        indoc! {"
            000000001
            000000000
            000000000
            000000000
            000000000
            000000000
            000000000
            000000000
            000000000
        "}
    )]
    #[case(
        Square::SQ_65.into(),
        indoc! {"
            000000000
            000000000
            000000000
            000000000
            000100000
            000000000
            000000000
            000000000
            000000000
        "}
    )]
    fn to_string(#[case] bb: Bitboard, #[case] expected: &str) {
        assert_eq!(bb.to_string(), expected);
    }
}
