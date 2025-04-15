// First things first generating the key. This is done by dropping
// every 8th character of the key, thus getting a 56bit key that is actually 
// used for the encryption

const KEY_SHIFT: [u8; 56] = [57, 49,  41,  33,  25,  17,   9,
                              1, 58,  50,  42,  34,  26,  18,
                             10,  2,  59,  51,  43,  35,  27,
                             19, 11,   3,  60,  52,  44,  36,
                             63, 55,  47,  39,  31,  23,  15,
                              7, 62,  54,  46,  38,  30,  22,
                             14,  6,  61,  53,  45,  37,  29,
                             21, 13,   5,  28,  20,  12,   4];

const SHIFTS: [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

const COMPRESSION: [u8; 48] =[14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10,
                               23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2,
                               41, 52, 31, 37, 47, 55, 30, 40, 51, 45, 33, 48,
                               44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32];

pub fn find_key(key: Vec<u8>) -> Vec<u8> {
    let mut new_key = Vec::new();

    for i in 0..56 {
        new_key.push(key[(KEY_SHIFT[i]-1) as usize]);
    }
    new_key
}

// Then we have to split the input word into two words, 32 bits each
pub fn split_vec(letters: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
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
pub fn bit_shift(key: Vec<u8>, round: usize) -> Vec<u8> {
    let shift = SHIFTS[round] as usize;     // Indices must be usize*
    let mut shifted_key = Vec::new();
    let len = key.len();
    
    for i in 0..key.len() {
        shifted_key.push(key[((i+len+shift)%len) as usize]);
    }
    shifted_key
}

// Next thing is the compression thing on the key
pub fn compress(key: Vec<u8>) -> Vec<u8> {
    let mut compressed = Vec::new();

    for i in 0..48 {
        compressed.push(key[(COMPRESSION[i]-1) as usize]);
    }
    compressed
}