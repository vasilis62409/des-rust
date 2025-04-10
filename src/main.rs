// Encrypting using DES
// One thing i think isn't necesary is moving all strings to vectors

// Initial Permutation table
const IP: [u32; 64] = [58, 50, 42, 34, 26, 18, 10, 2,
                       60, 52, 44, 36, 28, 20, 12, 4,
                       62, 54, 46, 38, 30, 22, 14, 6,
                       64, 56, 48, 40, 32, 24, 16, 8,
                       57, 49, 41, 33, 25, 17, 9, 1,
                       59, 51, 43, 35, 27, 19, 11, 3,
                       61, 53, 45, 37, 29, 21, 13, 5,
                       63, 55, 47, 39, 31, 23, 15, 7];

const KEY_SHIFT: [u8; 56] = [57, 49,  41,  33,  25,  17,   9,
                              1, 58,  50,  42,  34,  26,  18,
                             10,  2,  59,  51,  43,  35,  27,
                             19, 11,   3,  60,  52,  44,  36,
                             63, 55,  47,  39,  31,  23,  15,
                              7, 62,  54,  46,  38,  30,  22,
                             14,  6,  61,  53,  45,  37,  29,
                             21, 13,   5,  28,  20,  12,   4];

// Key shifts table
const SHIFTS: [u32; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

// Key compression permutation
const COMPRESSION: [u32; 48] =[14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10,
                               23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2,
                               41, 52, 31, 37, 47, 55, 30, 40, 51, 45, 33, 48,
                               44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32];

const EXPANSION: [u32; 48] = [31,  0,  1,  2,  3,  4, 
                               3,  4,  5,  6,  7,  8,
                               7,  8,  9, 10, 11, 12,
                              11, 12, 13, 14, 15, 16,
                              15, 16, 17, 18, 19, 20,
                              19, 20, 21, 22, 23, 24,
                              23, 24, 25, 26, 27, 28,
                              27, 28, 29, 30, 31,  0 ];

//Defining the s-boxes (took them from cahtgpt)
static S_BOXES: [[[u32; 16]; 4]; 8] = [
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
        [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 9, 5, 0, 14, 12],
    ],
    // S3
    [
        [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
        [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 15, 11, 12, 1],
        [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
        [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 12, 5, 2, 11],
    ],
    // S4
    [
        [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
        [13, 8, 11, 5, 6, 15, 0, 3, 14, 9, 10, 1, 7, 4, 2, 12],
        [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 14, 11, 9, 0, 2, 6],
        [10, 8, 0, 14, 6, 11, 13, 3, 15, 1, 7, 4, 9, 5, 2, 12],
    ],
    // S5
    [
        [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
        [14, 11, 2, 12, 4, 7, 13, 1, 5, 15, 10, 3, 9, 8, 6, 0],
        [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
        [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 9, 0, 5, 10, 3, 4],
    ],
    // S6
    [
        [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 14, 5, 11, 4, 7],
        [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
        [7, 11, 15, 1, 9, 14, 2, 8, 13, 12, 4, 5, 3, 10, 6, 0],
        [9, 14, 3, 4, 10, 7, 5, 15, 2, 8, 12, 1, 13, 0, 6, 11],
    ],
    // S7
    [
        [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
        [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
        [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 9, 5, 0, 6, 8, 2],
        [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
    ],
    // S8
    [
        [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
        [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
        [7, 11, 4, 1, 9, 12, 14, 2, 0, 8, 15, 13, 3, 10, 5, 6],
        [9, 7, 3, 13, 15, 2, 8, 14, 12, 4, 10, 11, 5, 0, 6, 1],
    ],
];

static P_BOX:[u32; 32] = [16,  7, 20, 21,
                          29, 12, 28, 17,
                           1, 15, 23, 26,
                           5, 18, 31, 10,
                           2,  8, 24, 14,
                          32, 27,  3,  9,
                          19, 13, 30,  6,
                          22, 11,  4, 25];

// First things first generating the key. This is done by dropping
// every 8th character of the key, thus getting a 56bit key that is actually 
// used for the encryption
fn find_key(key: Vec<u8>) -> Vec<u8> {
    let mut new_key = Vec::new();

    for i in 0..56 {
        new_key.push(key[(KEY_SHIFT[i]-1) as usize]);
    }
    new_key
}

// Then we have to split the input word into two words, 32 bits each
fn split_vec(letters: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let n = letters.len();
    let mut right = Vec::new();
    let mut left = Vec::new();

    for i in 0..n {
        if i < n/2 {
            left.push(letters[i]);
        }
        else {
            right.push(letters[i]);
        }
    }

    (left, right)
}

// Before we compress we need to shift the bits in the key
// I don't like this implementation, I'll think of something better
fn bit_shift(key: Vec<u8>, round: usize) -> Vec<u8> {
    let shift = SHIFTS[round] as usize;     // Indices must be usize*
    let mut shifted_key = Vec::new();
    let len = key.len();
    
    for i in 0..key.len() {
        shifted_key.push(key[((i+len+shift)%len) as usize]);
    }
    shifted_key
}

// Next thing is the compression thing on the key
fn compress(key: Vec<u8>) -> Vec<u8> {
    let mut compressed = Vec::new();

    for i in 0..48 {
        compressed.push(key[(COMPRESSION[i]-1) as usize]);
    }
    compressed
}

// Now the message. 
// First is the initial permutation function that acts, based on the table,
// on the input word. 
fn permutation(word: Vec<u8>) -> Vec<u8> {
    let mut init_p = Vec::new();

    for i in 0..word.len() {
        init_p.push(word[(IP[i]-1) as usize]);
    }   
    init_p
}

// Next we split the message into two halves. For this we use the same split_vec function as for the key
// After that we have the expansion on the RPT
fn expand(word: Vec<u8>) -> Vec<u8> {
    let mut expanded = Vec::new();

    for i in 0..48 {
        expanded.push(word[(EXPANSION[i]) as usize]);
    }
    expanded
}

// Now combine key and RPT to get a permutation
fn combination(key: Vec<u8>, rpt: Vec<u8>) -> Vec<u8> {
    let mut new_rpt = Vec::new();

    for i in 0..key.len() {
        new_rpt.push(key[i] ^ rpt[i]);
    }
    new_rpt
}

// S-boxes on the outcome of the last function
// need to fix the powers of 2
fn s_box_trans(word: Vec<u8>, round: usize) -> Vec<u32> {
    let mut transformed = Vec::new();
    let mut col = 0_u32;
    let mut row = 0_u32;

    for i in 0..8{
        for j in 0..6 {
            // One of these two is shit. Need to fix
            if(j % 6 == 0) | (j % 6 == 5) {
                row = row + (word[j+i*6]) as u32*(2_i32.pow((j%2).try_into().unwrap())) as u32;
                println!("{:?}, {:?}: {:?}",j, j+i*6, col);
            }
            else {
                col = col + (word[j+i*6]) as u32*(2_u8.pow((j-1).try_into().unwrap())) as u32;
                println!("{:?}, {:?}: {:?}",j, j+i*6, row);
            }
        }
        transformed.push(S_BOXES[round][row as usize][col as usize]);
        row = 0;
        col = 0;
    }
    transformed
}

// We conactenate the two halves

// Next is the P-box transformation

// Now the repeating part
// fn encryption(word: Vec:<u8>, key: Vec<u8>) -> Vec<u8> {
//     let new_key = find_key(key);
//     let mut cut_key = compress(new_key);
//     let (mut left, mut right) = split_vec(word);

//     for i in 0..16 {
//         let mut right_expanded = expand(right);
//         for i in 0..cut_key.len() {
//             right_expanded[i] = right_expanded[i] ^ cut_key[i]
//         }
//     }
//     right_expanded
// }

fn main() {
    let word = vec![0,0,0,1,0,0,1,0,
                    0,0,1,1,0,1,0,0,
                    0,1,0,1,0,1,1,0,
                    1,0,1,0,1,0,1,1,
                    1,1,0,0,1,1,0,1,
                    0,0,0,1,0,0,1,1,
                    0,0,1,0,0,1,0,1,
                    0,0,1,1,0,1,1,0];
    let key = vec![1,0,1,0,1,0,1,0,
                   1,0,1,1,1,0,1,1,
                   0,0,0,0,1,0,0,1,
                   0,0,0,1,1,0,0,0,
                   0,0,1,0,0,1,1,1,
                   0,0,1,1,0,1,1,0,
                   1,1,0,0,1,1,0,0,
                   1,1,0,1,1,1,0,1];

    //let (mut left, mut right) = split_vec(word.clone());

    let shuffled_key = find_key(key.clone());
    println!("PC1 key is: {:?}, and is {:?} bits long", shuffled_key, shuffled_key.len());
    let (mut left_key, mut right_key) = split_vec(shuffled_key.clone());
    println!("left key 0 is: {:?}", left_key);
    println!("right key 0 is: {:?}", right_key);
    (left_key, right_key) = (bit_shift(left_key, 1), bit_shift(right_key, 1));
    println!("left key is: {:?} and is {:?} bits long", left_key, left_key.len());
    println!("right key is: {:?} and is {:?} bits long", right_key, right_key.len());
    left_key.append(&mut right_key);
    let new_key56 = left_key.clone();
    println!("concatenated 56 bit key is: {:?}, and is {:?} bits long", new_key56, new_key56.len());
    let compressed_key = compress(new_key56);
    println!("compressed 56 key is: {:?} and is {:?} bits long", compressed_key, compressed_key.len());
    let test_s = s_box_trans(word, 1);
    println!("Test for s_box: {:?}", test_s);
}
