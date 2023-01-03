// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;
use std::ptr::hash;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_map = magazine.iter().fold(HashMap::new(), |mut map, word| {
        map.entry(word).and_modify(|count| *count += 1).or_insert(1);
        map
    });

    let note_map = note.iter().fold(HashMap::new(), |mut map, word| {
        map.entry(word).and_modify(|count| *count += 1).or_insert(1);
        map
    });

    note_map.iter().all(|(word, count)| magazine_map.get(word).unwrap_or(&0) >= count)
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
