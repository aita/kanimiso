/// Represents the distinct piece types in a game of Shogi.
///
/// Each variant corresponds to a different kind of piece, and they each have
/// their own unique movements and capture rules. Here's a list of the variants:
/// - `Pawn`: Represents a pawn, known as "Fu" in Japanese.
/// - `Lance`: Represents a lance, known as "Kyō" in Japanese.
/// - `Knight`: Represents a knight, known as "Kei" in Japanese.
/// - `Silver`: Represents a silver general, known as "Gin" in Japanese.
/// - `Bishop`: Represents a bishop, known as "Kaku" in Japanese.
/// - `Rook`: Represents a rook, known as "Hisya" in Japanese.
/// - `Gold`: Represents a gold general, known as "Kin" in Japanese.
/// - `King`: Represents a king, known as "Gyoku" or "Ō" in Japanese.
///
/// The enum also includes promoted variants of these pieces:
/// - `ProPawn`: A promoted pawn, known as "To" in Japanese.
/// - `ProLance`: A promoted lance, known as "Narikyō" in Japanese.
/// - `ProKnight`: A promoted knight, known as "Narikei" in Japanese.
/// - `ProSilver`: A promoted silver general, known as "Narigin" in Japanese.
/// - `Horse`: A promoted bishop, known as "Uma" in Japanese.
/// - `Dragon`: A promoted rook, known as "Ryū" in Japanese.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Lance,
    Knight,
    Silver,
    Bishop,
    Rook,
    Gold,
    King,
    ProPawn = Self::Pawn as isize | Self::PROMOTION_MASK,
    ProLance = Self::Lance as isize | Self::PROMOTION_MASK,
    ProKnight = Self::Knight as isize | Self::PROMOTION_MASK,
    ProSilver = Self::Silver as isize | Self::PROMOTION_MASK,
    Horse = Self::Bishop as isize | Self::PROMOTION_MASK,
    Dragon = Self::Rook as isize | Self::PROMOTION_MASK,
}

impl PieceKind {
    pub const COUNT: usize = 14;

    const PROMOTION_MASK: isize = 0x08;
    const PROMOTION_MASK_U8: u8 = Self::PROMOTION_MASK as u8;
    const WHITE_MASK: isize = 0x10;
    const WHITE_MASK_U8: u8 = Self::WHITE_MASK as u8;
    const PIECE_KIND_MASK: isize = 0x0F;
    const PIECE_KIND_MASK_U8: u8 = Self::PIECE_KIND_MASK as u8;

    #[inline(always)]
    pub fn is_promoted(&self) -> bool {
        (*self as u8) & Self::PROMOTION_MASK_U8 != 0
    }

    pub fn promote(&self) -> Option<Self> {
        Some(match self {
            Self::Pawn | Self::Lance | Self::Knight | Self::Silver | Self::Bishop | Self::Rook => {
                Self::from((*self as u8) | Self::PROMOTION_MASK_U8)
            }
            _ => return None,
        })
    }
}

impl From<u8> for PieceKind {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Pawn,
            1 => Self::Lance,
            2 => Self::Knight,
            3 => Self::Silver,
            4 => Self::Bishop,
            5 => Self::Rook,
            6 => Self::Gold,
            7 => Self::King,
            8 => Self::ProPawn,
            9 => Self::ProLance,
            10 => Self::ProKnight,
            11 => Self::ProSilver,
            12 => Self::Horse,
            13 => Self::Dragon,
            _ => panic!(
                "Invalid value for PieceKind: {}. Please ensure it is between 0 and 13 inclusive.",
                value
            ),
        }
    }
}

/// Represents the two players' sides in a game of Shogi.
///
/// Each player in the game is assigned one of these colors at the start of the game.
/// The player with the black pieces typically goes first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub const COUNT: usize = 2;
}

/// Represents individual pieces in a game of Shogi.
///
/// Each variant of the enum corresponds to a different piece and which player it belongs to.
/// Variants starting with "B" represent pieces for the black player, and variants starting with "W" represent pieces for the white player.
///
/// Here's the meaning of each variant:
/// - `BPawn`, `WPawn`: A pawn belonging to the black or white player, respectively.
/// - `BLance`, `WLance`: A lance belonging to the black or white player, respectively.
/// - `BKnight`, `WKnight`: A knight belonging to the black or white player, respectively.
/// - `BSilver`, `WSilver`: A silver general belonging to the black or white player, respectively.
/// - `BBishop`, `WBishop`: A bishop belonging to the black or white player, respectively.
/// - `BRook`, `WRook`: A rook belonging to the black or white player, respectively.
/// - `BGold`, `WGold`: A gold general belonging to the black or white player, respectively.
/// - `BKing`, `WKing`: A king belonging to the black or white player, respectively.
///
/// The enum also includes promoted variants of these pieces for each player:
/// - `BProPawn`, `WProPawn`: A promoted pawn for the black or white player, respectively.
/// - `BProLance`, `WProLance`: A promoted lance for the black or white player, respectively.
/// - `BProKnight`, `WProKnight`: A promoted knight for the black or white player, respectively.
/// - `BProSilver`, `WProSilver`: A promoted silver general for the black or white player, respectively.
/// - `BHorse`, `WHorse`: A promoted bishop for the black or white player, respectively.
/// - `BDragon`, `WDragon`: A promoted rook for the black or white player, respectively.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    BPawn = PieceKind::Pawn as isize,
    BLance = PieceKind::Lance as isize,
    BKnight = PieceKind::Knight as isize,
    BSilver = PieceKind::Silver as isize,
    BBishop = PieceKind::Bishop as isize,
    BRook = PieceKind::Rook as isize,
    BGold = PieceKind::Gold as isize,
    BKing = PieceKind::King as isize,
    BProPawn = PieceKind::ProPawn as isize,
    BProLance = PieceKind::ProLance as isize,
    BProKnight = PieceKind::ProKnight as isize,
    BProSilver = PieceKind::ProSilver as isize,
    BHorse = PieceKind::Horse as isize,
    BDragon = PieceKind::Dragon as isize,
    WPawn = PieceKind::Pawn as isize | PieceKind::WHITE_MASK,
    WLance = PieceKind::Lance as isize | PieceKind::WHITE_MASK,
    WKnight = PieceKind::Knight as isize | PieceKind::WHITE_MASK,
    WSilver = PieceKind::Silver as isize | PieceKind::WHITE_MASK,
    WBishop = PieceKind::Bishop as isize | PieceKind::WHITE_MASK,
    WRook = PieceKind::Rook as isize | PieceKind::WHITE_MASK,
    WGold = PieceKind::Gold as isize | PieceKind::WHITE_MASK,
    WKing = PieceKind::King as isize | PieceKind::WHITE_MASK,
    WProPawn = PieceKind::ProPawn as isize | PieceKind::WHITE_MASK,
    WProLance = PieceKind::ProLance as isize | PieceKind::WHITE_MASK,
    WProKnight = PieceKind::ProKnight as isize | PieceKind::WHITE_MASK,
    WProSilver = PieceKind::ProSilver as isize | PieceKind::WHITE_MASK,
    WHorse = PieceKind::Horse as isize | PieceKind::WHITE_MASK,
    WDragon = PieceKind::Dragon as isize | PieceKind::WHITE_MASK,
}

impl Piece {
    pub const COUNT: usize = 28;

    pub fn new(color: Color, piece_kind: PieceKind) -> Self {
        let color_mask = match color {
            Color::Black => 0,
            Color::White => PieceKind::WHITE_MASK_U8,
        };
        Piece::from(piece_kind as u8 | color_mask)
    }

    #[inline(always)]
    pub fn kind(&self) -> PieceKind {
        let kind = (*self as u8) & PieceKind::PIECE_KIND_MASK_U8;
        PieceKind::from(kind)
    }

    #[inline(always)]
    pub fn is_black(&self) -> bool {
        (*self as u8) & PieceKind::WHITE_MASK_U8 == 0
    }

    #[inline(always)]
    pub fn is_white(&self) -> bool {
        (*self as u8) & PieceKind::WHITE_MASK_U8 != 0
    }

    #[inline(always)]
    pub fn color(&self) -> Color {
        if self.is_black() {
            Color::Black
        } else {
            Color::White
        }
    }

    #[inline(always)]
    pub fn is_promoted(&self) -> bool {
        (*self as u8) & PieceKind::PROMOTION_MASK_U8 != 0
    }

    pub fn promote(&self) -> Option<Self> {
        let kind = self.kind();
        kind.promote().map(|kind| Self::new(self.color(), kind))
    }
}

impl From<u8> for Piece {
    fn from(value: u8) -> Self {
        match value {
            0 => Piece::BPawn,
            1 => Piece::BLance,
            2 => Piece::BKnight,
            3 => Piece::BSilver,
            4 => Piece::BBishop,
            5 => Piece::BRook,
            6 => Piece::BGold,
            7 => Piece::BKing,
            8 => Piece::BProPawn,
            9 => Piece::BProLance,
            10 => Piece::BProKnight,
            11 => Piece::BProSilver,
            12 => Piece::BHorse,
            13 => Piece::BDragon,
            16 => Piece::WPawn,
            17 => Piece::WLance,
            18 => Piece::WKnight,
            19 => Piece::WSilver,
            20 => Piece::WBishop,
            21 => Piece::WRook,
            22 => Piece::WGold,
            23 => Piece::WKing,
            24 => Piece::WProPawn,
            25 => Piece::WProLance,
            26 => Piece::WProKnight,
            27 => Piece::WProSilver,
            28 => Piece::WHorse,
            29 => Piece::WDragon,
            _ => panic!(
                "Invalid value for Piece: {}. Please ensure it is between 0 and 29 inclusive.",
                value
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn new() {
        for color in [Color::Black, Color::White] {
            for piece_kind in 0..PieceKind::COUNT as u8 {
                let piece_kind = PieceKind::from(piece_kind);
                let piece = Piece::new(color, piece_kind);

                assert_eq!(piece.kind(), piece_kind);
                assert_eq!(piece.color(), color);
            }
        }
    }

    #[test]
    fn color() {
        for color in [Color::Black, Color::White] {
            for piece_kind in 0..PieceKind::COUNT as u8 {
                let piece_kind = PieceKind::from(piece_kind);
                let piece = Piece::new(color, piece_kind);

                match color {
                    Color::Black => {
                        assert!(piece.is_black());
                        assert!(!piece.is_white());
                        assert_eq!(piece.color(), Color::Black);
                    }
                    Color::White => {
                        assert!(!piece.is_black());
                        assert!(piece.is_white());
                        assert_eq!(piece.color(), Color::White);
                    }
                }
            }
        }
    }

    #[rstest]
    #[case(Piece::BPawn, false, Some(Piece::BProPawn))]
    #[case(Piece::BLance, false, Some(Piece::BProLance))]
    #[case(Piece::BKnight, false, Some(Piece::BProKnight))]
    #[case(Piece::BSilver, false, Some(Piece::BProSilver))]
    #[case(Piece::BBishop, false, Some(Piece::BHorse))]
    #[case(Piece::BRook, false, Some(Piece::BDragon))]
    #[case(Piece::BGold, false, None)]
    #[case(Piece::BKing, false, None)]
    #[case(Piece::BProPawn, true, None)]
    #[case(Piece::BProLance, true, None)]
    #[case(Piece::BProKnight, true, None)]
    #[case(Piece::BProSilver, true, None)]
    #[case(Piece::BHorse, true, None)]
    #[case(Piece::BDragon, true, None)]
    #[case(Piece::WPawn, false, Some(Piece::WProPawn))]
    #[case(Piece::WLance, false, Some(Piece::WProLance))]
    #[case(Piece::WKnight, false, Some(Piece::WProKnight))]
    #[case(Piece::WSilver, false, Some(Piece::WProSilver))]
    #[case(Piece::WBishop, false, Some(Piece::WHorse))]
    #[case(Piece::WRook, false, Some(Piece::WDragon))]
    #[case(Piece::WGold, false, None)]
    #[case(Piece::WKing, false, None)]
    #[case(Piece::WProPawn, true, None)]
    #[case(Piece::WProLance, true, None)]
    #[case(Piece::WProKnight, true, None)]
    #[case(Piece::WProSilver, true, None)]
    #[case(Piece::WHorse, true, None)]
    #[case(Piece::WDragon, true, None)]
    fn promote(
        #[case] piece: Piece,
        #[case] is_promoted: bool,
        #[case] promoted_piece: Option<Piece>,
    ) {
        assert_eq!(piece.is_promoted(), is_promoted);
        assert_eq!(piece.promote(), promoted_piece);

        assert_eq!(piece.kind().is_promoted(), is_promoted);
        assert_eq!(piece.kind().promote(), promoted_piece.map(|p| p.kind()));
    }
}
