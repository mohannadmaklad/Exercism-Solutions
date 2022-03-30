// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut hm = HashMap::new();

    //Construct the hash map
    for word in magazine {
        *hm.entry(word).or_insert(0) += 1;
    }

    //Remove words
    for word in note {
        *hm.entry(word).or_insert(0) -= 1;
        if hm[word] < 0 {
            return false;
        }
    }

    true
}
