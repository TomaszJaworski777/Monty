#[cfg(feature = "pext")]
use std::arch::x86_64::_pext_u64;

#[cfg(not(feature = "pext"))]
const BISHOP_ATTACKS: [u64; 512 * 64] = unsafe { std::mem::transmute(*include_bytes!("tables/bishop_attacks.bin")) };
#[cfg(feature = "pext")]
const BISHOP_ATTACKS: [u64; 512 * 64] = unsafe { std::mem::transmute(*include_bytes!("tables/bishop_attacks_pext.bin")) };

pub struct BishopAttacks;
impl BishopAttacks {
    #[inline]
    pub fn get_bishop_attacks(square: usize, occupancy: u64) -> u64 {
        let square = usize::from(square);

        #[cfg(not(feature = "pext"))]
        let (mask, shift, magic) = BISHOP_MAGICS[square];

        #[cfg(not(feature = "pext"))]
        let index = ((occupancy & mask)
            .wrapping_mul(magic)
            >> shift) as usize;

        #[cfg(feature = "pext")]
        let index =
            unsafe { _pext_u64(occupancy, BISHOP_MASKS[square]) as usize };

        BISHOP_ATTACKS[(square * 512) + index]
    }
}

#[cfg(not(feature = "pext"))]
const BISHOP_MAGICS: [(u64, u32, u64); 64] = {
    let mut result = [(0, 0, 0); 64];
    let mut square_index = 0usize;
    while square_index < 64 {
        result[square_index] = (BISHOP_MASKS[square_index], 64 - BISHOP_OCCUPANCY_COUNT[square_index] as u32, MAGIC_NUMBERS_BISHOP[square_index]);
        square_index += 1;
    }

    result
};

const BISHOP_MASKS: [u64; 64] = {
    let mut result = [0; 64];
    let mut square_index = 0;
    while square_index < 64 {
        result[square_index as usize] = mask_bishop_attacks(square_index);
        square_index += 1;
    }
    result
};

#[cfg(not(feature = "pext"))]
const BISHOP_OCCUPANCY_COUNT: [usize; 64] = {
    let mut result = [0; 64];
    let mut rank = 0;
    while rank < 8 {
        let mut file = 0;
        while file < 8 {
            let square = rank * 8 + file;
            result[square] = mask_bishop_attacks(square).count_ones() as usize;
            file += 1;
        }
        rank += 1;
    }
    result
};

const fn mask_bishop_attacks(square: usize) -> u64 {
    let mut result: u64 = 0;
    let bishop_position = (square as i32 / 8, square as i32 % 8);

    let mut rank = bishop_position.0 + 1;
    let mut file = bishop_position.1 + 1;
    while rank < 7 && file < 7 {
        result |= 1u64 << (rank * 8 + file);
        rank += 1;
        file += 1;
    }

    rank = bishop_position.0 - 1;
    file = bishop_position.1 + 1;
    while rank > 0 && file < 7 {
        result |= 1u64 << (rank * 8 + file);
        rank -= 1;
        file += 1;
    }

    rank = bishop_position.0 - 1;
    file = bishop_position.1 - 1;
    while rank > 0 && file > 0 {
        result |= 1u64 << (rank * 8 + file);
        rank -= 1;
        file -= 1;
    }

    rank = bishop_position.0 + 1;
    file = bishop_position.1 - 1;
    while rank < 7 && file > 0 {
        result |= 1u64 << (rank * 8 + file);
        rank += 1;
        file -= 1;
    }

    result
}

#[cfg(not(feature = "pext"))]
const MAGIC_NUMBERS_BISHOP: [u64; 64] = [
    9300092178686681120,
    1284830893973760,
    2322997520105472,
    16142031364873674789,
    10383348832699154706,
    571763293421568,
    37726495118197760,
    1513231473652670722,
    40550006146990185,
    873700543932137730,
    36037870288505856,
    431188982368272,
    1155210765395821056,
    11538293718411908608,
    4611721787053966849,
    103589390848170272,
    1125968899098624,
    144680358661721088,
    11259553153024529,
    10133272101128193,
    73751202732572676,
    144679238632472832,
    2357915965813425297,
    401383670122021888,
    13528392142225729,
    4643215615211930112,
    9226802530447557664,
    1302666467161997954,
    1306326466426847232,
    2253998841200772,
    9223935538715955216,
    4611977389012961280,
    1161101459345408,
    5630633405878272,
    154573777173479968,
    5224739618297217088,
    184790590020518016,
    141291540840712,
    4621296042111943168,
    9278545841721754664,
    13814550243590400,
    757176487873905668,
    2598717998437009408,
    2344123889522575360,
    360290349769303040,
    14053484853547533328,
    9227878118977438752,
    5102361295591936,
    5233754530306591776,
    4689658989384957952,
    1161642645719051,
    2252351784355840,
    2337004390424,
    1190112502864707589,
    290499785468691593,
    2387190454312566784,
    1235149585505599557,
    4683745820179825441,
    18014407116507136,
    1741698094928005,
    144749056665649409,
    576461028523640968,
    74921813755137,
    18085875364200714,
];
