use std::fmt::Debug;

/// Represents a square on a Shogi board.
///
/// In Shogi, the board is a grid of 9x9 squares, so each `Square`
/// has two properties: `file` and `rank`. These represent the horizontal
/// and vertical coordinates of the square on the board respectively.
///
/// The `file` value ranges from 1 to 9, moving from right (file 1) to left (file 9) across the board.
/// Similarly, the `rank` value ranges from 1 to 9, moving from the top (rank 1) to the bottom (rank 9) of the board.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Square(u8);

impl Square {
    pub fn from_coord(file: u8, rank: u8) -> Self {
        assert!(
            file < 9 && rank < 9,
            "The values for file and rank must be in the range 0 to 9 inclusive. Current file value is {} and rank value is {}.",
            file,
            rank
        );

        Self(file * 9 + rank)
    }

    #[inline(always)]
    pub fn file(&self) -> u8 {
        self.0 / 9
    }

    #[inline(always)]
    pub fn rank(&self) -> u8 {
        self.0 % 9
    }

    #[inline(always)]
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.file();
        let rank = self.rank();
        write!(f, "SQ_{}{}", file + 1, rank + 1)
    }
}

macro_rules! const_square {
    ($name:ident, $value:expr) => {
        pub const $name: Square = Square($value);
    };
}

macro_rules! const_squares {
    ($name:ident = $value:expr, $($names:ident),*) => {
        const_square!($name, $value);
        const_squares!($value + 1, $($names),*);
    };

    ($value:expr, $name:ident, $($names:ident),+) => {
        const_square!($name, $value);
        const_squares!($value + 1, $($names),*);
    };

    ($value:expr, $name:ident) => {
        const_square!($name, $value);
    };
}

impl Square {
    pub const COUNT: usize = 81;

    const_squares! {
        SQ_11 = 0,
        SQ_12,
        SQ_13,
        SQ_14,
        SQ_15,
        SQ_16,
        SQ_17,
        SQ_18,
        SQ_19,
        SQ_21,
        SQ_22,
        SQ_23,
        SQ_24,
        SQ_25,
        SQ_26,
        SQ_27,
        SQ_28,
        SQ_29,
        SQ_31,
        SQ_32,
        SQ_33,
        SQ_34,
        SQ_35,
        SQ_36,
        SQ_37,
        SQ_38,
        SQ_39,
        SQ_41,
        SQ_42,
        SQ_43,
        SQ_44,
        SQ_45,
        SQ_46,
        SQ_47,
        SQ_48,
        SQ_49,
        SQ_51,
        SQ_52,
        SQ_53,
        SQ_54,
        SQ_55,
        SQ_56,
        SQ_57,
        SQ_58,
        SQ_59,
        SQ_61,
        SQ_62,
        SQ_63,
        SQ_64,
        SQ_65,
        SQ_66,
        SQ_67,
        SQ_68,
        SQ_69,
        SQ_71,
        SQ_72,
        SQ_73,
        SQ_74,
        SQ_75,
        SQ_76,
        SQ_77,
        SQ_78,
        SQ_79,
        SQ_81,
        SQ_82,
        SQ_83,
        SQ_84,
        SQ_85,
        SQ_86,
        SQ_87,
        SQ_88,
        SQ_89,
        SQ_91,
        SQ_92,
        SQ_93,
        SQ_94,
        SQ_95,
        SQ_96,
        SQ_97,
        SQ_98,
        SQ_99
    }
}
