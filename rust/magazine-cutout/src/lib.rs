// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;
use std::ptr::hash;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_map = hashmap_from(magazine);
    let note_map = hashmap_from(note);

    for (word, note_count) in note_map {
        match magazine_map.get(word) {
            Some(&magazine_count) if magazine_count >= note_count => {}
            _ => return false
        }
    }

    true
}

fn hashmap_from<'a>(words: &[&'a str]) -> HashMap<&'a str, u32> {
    let mut map = HashMap::new();

    for &word in words {
        map.entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    map
}
