// Now the message. 
// First is the initial permutation function that acts, based on the table,
// on the input word. 

const IP: [u8; 64] = [58, 50, 42, 34, 26, 18, 10, 2,
                       60, 52, 44, 36, 28, 20, 12, 4,
                       62, 54, 46, 38, 30, 22, 14, 6,
                       64, 56, 48, 40, 32, 24, 16, 8,
                       57, 49, 41, 33, 25, 17, 9, 1,
                       59, 51, 43, 35, 27, 19, 11, 3,
                       61, 53, 45, 37, 29, 21, 13, 5,
                       63, 55, 47, 39, 31, 23, 15, 7];

const EXPANSION: [u8; 48] = [31,  0,  1,  2,  3,  4, 
                               3,  4,  5,  6,  7,  8,
                               7,  8,  9, 10, 11, 12,
                              11, 12, 13, 14, 15, 16,
                              15, 16, 17, 18, 19, 20,
                              19, 20, 21, 22, 23, 24,
                              23, 24, 25, 26, 27, 28,
                              27, 28, 29, 30, 31,  0 ];

static S_BOXES: [[[u8; 16]; 4]; 8] = [
    // S1
    [
        [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
        [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
        [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
        [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
    ],
    // S2
    [
        [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
        [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
        [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
        [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
    ],
    // S3
    [
        [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
        [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
        [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
        [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
    ],
    // S4
    [
        [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
        [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
        [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
        [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
    ],
    // S5
    [
        [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
        [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
        [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
        [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
    ],
    // S6
    [
        [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
        [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
        [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
        [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
    ],
    // S7
    [
        [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
        [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
        [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
        [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
    ],
    // S8
    [
        [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
        [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
        [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
        [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],
    ],
];

static P_BOX:[u8; 32] = [16,  7, 20, 21,
                          29, 12, 28, 17,
                           1, 15, 23, 26,
                           5, 18, 31, 10,
                           2,  8, 24, 14,
                          32, 27,  3,  9,
                          19, 13, 30,  6,
                          22, 11,  4, 25];

static INVERSE_IP:[u8; 64] = [40,     8,   48,    16,    56,   24,    64,   32,
                     39,     7,   47,    15,    55,   23,    63,   31,
                     38,     6,   46,    14,    54,   22,    62,   30,
                     37,     5,   45,    13,    53,   21,    61,   29,
                     36,     4,   44,    12,    52,   20,    60,   28,
                     35,     3,   43,    11,    51,   19,    59,   27,
                     34,     2,   42,    10,    50,   18,    58,   26,
                     33,     1,   41,     9,    49,   17,    57,   25,];

pub fn permutation(word: &Vec<u8>) -> Vec<u8> {
    let mut init_p = Vec::new();

    for i in 0..word.len() {
        init_p.push(word[(IP[i]-1) as usize]);
    }   
    init_p
}

// Next we split the message into two halves. For this we use the same split_vec function as for the key
// After that we have the expansion on the RPT
pub fn expand(word: &Vec<u8>) -> Vec<u8> {
    let mut expanded = Vec::new();

    for i in 0..48 {
        expanded.push(word[(EXPANSION[i]) as usize]);
    }
    expanded
}

// Now combine key and RPT to get a permutation
pub fn combination(key: Vec<u8>, rpt: Vec<u8>) -> Vec<u8> {
    let mut new_rpt = Vec::new();

    for i in 0..key.len() {
        new_rpt.push(key[i] ^ rpt[i]);
    }
    new_rpt
}

// S-boxes on the outcome of the last function 
pub fn s_box_trans(word: Vec<u8>) -> Vec<u8> {
    let mut transformed = Vec::new();
    let mut col = 0_u32;
    let mut row = 0_u32;
    let mut out = 0_i32;
    let mut diff = 0_i32;

    for i in 0..8{
        for j in 0..6 {
            // One of these two is shit. Need to fix
            if(j % 6 == 0) | (j % 6 == 5) {
                row = row + (word[j+i*6]) as u32*(2_u32.pow(((j+1)%2).try_into().unwrap()));
            }
            else {
                let power = ((4-j) as u32)%4;
                col = col + (word[j+i*6]) as u32*(2_u32.pow((power).try_into().unwrap()));
            }
        }
        out = S_BOXES[i][row as usize][col as usize] as i32;
        for k in 0..4 {
            diff = (out-(2_i32.pow((3-k) % 4))) as i32;
            if diff<0 {
                transformed.push(0);
            }
            else{
                transformed.push(1);
                out = diff;
            }
        }
        row = 0;
        col = 0;
    }
    transformed
}

// Next is the P-box transformation
pub fn p_box_trans(word: Vec<u8>) -> Vec<u8> {
    let mut transformed = Vec::new();

    for i in 0..word.len() {
        transformed.push(word[(P_BOX[i]-1) as usize]);
    }
    transformed
}

// We conactenate the two halves
pub fn concat(lpt: Vec<u8>, rpt: Vec<u8>) -> Vec<u8>{
    let mut conced = Vec::new();

    for bit in rpt {
        conced.push(bit);
    }


    for bit in lpt {
        conced.push(bit);
    }
    conced
}

pub fn inverse_permutation(word: Vec<u8>) -> Vec<u8> {
    let mut init_p = Vec::new();

    for i in 0..word.len() {
        init_p.push(word[(INVERSE_IP[i]-1) as usize]);
    }   
    init_p
}