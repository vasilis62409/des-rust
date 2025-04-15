// Encrypting using DES

mod message_work;
mod key_work;

// The function that puts everything together... Finally
fn encrypt_message(word: Vec<u8>, key:Vec<u8>) -> Vec<u8> {
    let mut shuffled_key = key_work::find_key(key.clone());
    let (mut left_key, mut right_key) = key_work::split_vec(shuffled_key.clone());
    let mut new_key = Vec::new();
    let mut permuted_word = message_work::permutation(&word);
    let (mut left_word, mut right_word) = key_work::split_vec(permuted_word);
    let mut placeholder = Vec::new();
    let mut new_right_word = Vec::new();
    let mut new_word = Vec::new();

    for round in 0..16 {
        // split
        (left_key, right_key) = (key_work::bit_shift(left_key.clone(), round), key_work::bit_shift(right_key.clone(), round));
        // make concatenated key
        new_key = left_key.clone();
        new_key.append(&mut right_key.clone());
        // compress. This is the key for the round
        new_key = key_work::compress(new_key);

        // The message 
        new_right_word = message_work::expand(&right_word);
        println!("expansion: {:?}: ", new_right_word);
        new_right_word = message_work::combination(new_key, new_right_word);
        println!("xor1: {:?}: ", new_right_word);
        new_right_word = message_work::s_box_trans(new_right_word);
        println!("S box: {:?}: ", new_right_word);
        new_right_word = message_work::p_box_trans(new_right_word);
        println!("P box: {:?}: ", new_right_word);

        placeholder = right_word;
        right_word = message_work::combination(left_word, new_right_word);
        left_word = placeholder;
        //println!("right word: {:?}, left word: {:?}:", right_word, left_word);
    }

    new_word = message_work::concat(left_word, right_word);
    message_work::inverse_permutation(new_word)
}

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

   println!("Encrypted word: {:?}", encrypt_message(word, key));
   // let test_s = message_work::p_box_trans(word);
   // println!("{:?}", test_s);
}
