#[cfg(feature = "pext")]
use std::arch::x86_64::_pext_u64;

#[cfg(not(feature = "pext"))]
const ROOK_ATTACKS: [u64; 4096 * 64] = unsafe { std::mem::transmute(*include_bytes!("tables/rook_attacks.bin")) };
#[cfg(feature = "pext")]
const ROOK_ATTACKS: [u64; 4096 * 64] = unsafe { std::mem::transmute(*include_bytes!("tables/rook_attacks_pext.bin")) };

pub struct RookAttacks;
impl RookAttacks {
    #[inline]
    pub fn get_rook_attacks(square: usize, occupancy: u64) -> u64 {
        
        #[cfg(not(feature = "pext"))]
        let (mask, shift, magic) = ROOK_MAGICS[square];

        #[cfg(not(feature = "pext"))]
        let index = ((occupancy & mask)
            .wrapping_mul(magic)
            >> shift) as usize;

        #[cfg(feature = "pext")]
        let index =
            unsafe { _pext_u64(occupancy, ROOK_MASKS[square]) as usize };

        ROOK_ATTACKS[(square * 4096) + index]
    }
}

#[cfg(not(feature = "pext"))]
const ROOK_MAGICS: [(u64, u32, u64); 64] = {
    let mut result = [(0, 0, 0); 64];
    let mut square_index = 0usize;
    while square_index < 64 {
        result[square_index] = (ROOK_MASKS[square_index], 64 - ROOK_OCCUPANCY_COUNT[square_index] as u32, MAGIC_NUMBERS_ROOK[square_index]);
        square_index += 1;
    }

    result
};


const ROOK_MASKS: [u64; 64] = {
    let mut result = [0; 64];
    let mut square_index = 0;
    while square_index < 64 {
        result[square_index as usize] = mask_rook_attacks(square_index);
        square_index += 1;
    }
    result
};

#[cfg(not(feature = "pext"))]
const ROOK_OCCUPANCY_COUNT: [usize; 64] = {
    let mut result = [0; 64];
    let mut rank = 0;
    while rank < 8 {
        let mut file = 0;
        while file < 8 {
            let square = rank * 8 + file;
            result[square] = mask_rook_attacks(square).count_ones() as usize;
            file += 1;
        }
        rank += 1;
    }
    result
};

const fn mask_rook_attacks(square: usize) -> u64 {
    let mut result: u64 = 0;
    let rook_position = (square as i32 / 8, square as i32 % 8);

    let mut rank = rook_position.0 + 1;
    let mut file = rook_position.1;
    while rank < 7 {
        result |= 1u64 << (rank * 8 + file);
        rank += 1;
    }

    rank = rook_position.0 - 1;
    file = rook_position.1;
    while rank > 0 {
        result |= 1u64 << (rank * 8 + file);
        rank -= 1;
    }

    rank = rook_position.0;
    file = rook_position.1 + 1;
    while file < 7 {
        result |= 1u64 << (rank * 8 + file);
        file += 1;
    }

    rank = rook_position.0;
    file = rook_position.1 - 1;
    while file > 0 {
        result |= 1u64 << (rank * 8 + file);
        file -= 1;
    }

    result
}

#[cfg(not(feature = "pext"))]
const MAGIC_NUMBERS_ROOK: [u64; 64] = [
    9259400973461241857,
    234187460333015040,
    36063981659521032,
    2377918195574046724,
    1080868867234332928,
    72061992118059016,
    180144534867411456,
    72058693558158370,
    5260345103070806016,
    1378171992426954752,
    13835199342794776576,
    90353536244130048,
    1155314059089281152,
    583356906421125632,
    562984346714500,
    585608691194020096,
    1188951126274211904,
    40550263712383040,
    144749606589170949,
    576762018642657345,
    4613938094192984576,
    1126449729896576,
    144116291882713600,
    1128099206989892,
    4908959330109243397,
    5764677945467601024,
    35186520621184,
    166650782695882760,
    4408784453760,
    9549885211018265600,
    18028709342085744,
    4423816397473,
    15024008631798472704,
    144185694263185412,
    9799938353053839360,
    4614078624457295873,
    578721350366004224,
    704795551728640,
    1729663887059452416,
    576461303166534673,
    9511672783898181668,
    9259488795341373440,
    153123487114919972,
    4503634054234176,
    144396697438584836,
    2199090397312,
    2395916444903931912,
    281476058906626,
    288275458347631104,
    14001693577961277760,
    1585284936444020224,
    5764748329242591872,
    22799490427785472,
    140746078552192,
    81346276859576576,
    325398273679442432,
    35257390760450,
    15908851192709121,
    8076117492512065602,
    148746468910469121,
    4653907677319540842,
    281509370265601,
    162130969700081796,
    1445673626624869378,
];
