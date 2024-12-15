use crate::init;

use super::{consts::Piece, sliders::{BishopAttacks, RookAttacks}};

pub struct Attacks;
impl Attacks {
    pub fn of_piece<const PC: usize>(from: usize, occ: u64) -> u64 {
        match PC {
            Piece::KNIGHT => Attacks::knight(from),
            Piece::BISHOP => Attacks::bishop(from, occ),
            Piece::ROOK => Attacks::rook(from, occ),
            Piece::QUEEN => Attacks::queen(from, occ),
            Piece::KING => Attacks::king(from),
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn pawn(sq: usize, side: usize) -> u64 {
        LOOKUP.pawn[side][sq]
    }

    #[inline]
    pub fn knight(sq: usize) -> u64 {
        LOOKUP.knight[sq]
    }

    #[inline]
    pub fn king(sq: usize) -> u64 {
        LOOKUP.king[sq]
    }

    // hyperbola quintessence
    // this gets automatically vectorised when targeting avx or better
    #[inline]
    pub fn bishop(sq: usize, occ: u64) -> u64 {
        BishopAttacks::get_bishop_attacks(sq, occ)
    }

    // shifted lookup
    // files and ranks are mapped to 1st rank and looked up by occupancy
    #[inline]
    pub fn rook(sq: usize, occ: u64) -> u64 {
        RookAttacks::get_rook_attacks(sq, occ)
    }

    #[inline]
    pub fn queen(sq: usize, occ: u64) -> u64 {
        Self::bishop(sq, occ) | Self::rook(sq, occ)
    }

    #[inline]
    pub fn xray_rook(sq: usize, occ: u64, blockers: u64) -> u64 {
        let attacks = Self::rook(sq, occ);
        attacks ^ Self::rook(sq, occ ^ (attacks & blockers))
    }

    #[inline]
    pub fn xray_bishop(sq: usize, occ: u64, blockers: u64) -> u64 {
        let attacks = Self::bishop(sq, occ);
        attacks ^ Self::bishop(sq, occ ^ (attacks & blockers))
    }

    pub const fn white_pawn_setwise(pawns: u64) -> u64 {
        ((pawns & !File::A) << 7) | ((pawns & !File::H) << 9)
    }

    pub const fn black_pawn_setwise(pawns: u64) -> u64 {
        ((pawns & !File::A) >> 9) | ((pawns & !File::H) >> 7)
    }

    pub const ALL_DESTINATIONS: [u64; 64] = init!(|sq, 64| {
        let rank = sq / 8;
        let file = sq % 8;

        let rooks = (0xFF << (rank * 8)) ^ (File::A << file);
        let bishops = DIAGS[file + rank].swap_bytes() ^ DIAGS[7 + file - rank];

        rooks | bishops | KNIGHT[sq] | KING[sq]
    });
}

struct File;
impl File {
    const A: u64 = 0x0101_0101_0101_0101;
    const H: u64 = Self::A << 7;
}

const DIAGS: [u64; 15] = [
    0x0100_0000_0000_0000,
    0x0201_0000_0000_0000,
    0x0402_0100_0000_0000,
    0x0804_0201_0000_0000,
    0x1008_0402_0100_0000,
    0x2010_0804_0201_0000,
    0x4020_1008_0402_0100,
    0x8040_2010_0804_0201,
    0x0080_4020_1008_0402,
    0x0000_8040_2010_0804,
    0x0000_0080_4020_1008,
    0x0000_0000_8040_2010,
    0x0000_0000_0080_4020,
    0x0000_0000_0000_8040,
    0x0000_0000_0000_0080,
];
struct Lookup {
    pawn: [[u64; 64]; 2],
    knight: [u64; 64],
    king: [u64; 64],
}

static LOOKUP: Lookup = Lookup {
    pawn: PAWN,
    knight: KNIGHT,
    king: KING,
};

const PAWN: [[u64; 64]; 2] = [
    init!(|sq, 64| (((1 << sq) & !File::A) << 7) | (((1 << sq) & !File::H) << 9)),
    init!(|sq, 64| (((1 << sq) & !File::A) >> 9) | (((1 << sq) & !File::H) >> 7)),
];

const KNIGHT: [u64; 64] = init!(|sq, 64| {
    let n = 1 << sq;
    let h1 = ((n >> 1) & 0x7f7f_7f7f_7f7f_7f7f) | ((n << 1) & 0xfefe_fefe_fefe_fefe);
    let h2 = ((n >> 2) & 0x3f3f_3f3f_3f3f_3f3f) | ((n << 2) & 0xfcfc_fcfc_fcfc_fcfc);
    (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
});

const KING: [u64; 64] = init!(|sq, 64| {
    let mut k = 1 << sq;
    k |= (k << 8) | (k >> 8);
    k |= ((k & !File::A) >> 1) | ((k & !File::H) << 1);
    k ^ (1 << sq)
});

pub const fn line_through(i: usize, j: usize) -> u64 {
    let sq = 1 << j;

    let rank = i / 8;
    let file = i % 8;

    let files = File::A << file;
    if files & sq > 0 {
        return files;
    }

    let ranks = 0xFF << (8 * rank);
    if ranks & sq > 0 {
        return ranks;
    }

    let diags = DIAGS[7 + file - rank];
    if diags & sq > 0 {
        return diags;
    }

    let antis = DIAGS[file + rank].swap_bytes();
    if antis & sq > 0 {
        return antis;
    }

    0
}
