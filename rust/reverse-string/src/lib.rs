use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
   input.graphemes(true).rev().collect()
}

// fn reverse_iterator(input: &str) -> String {
//     input.chars().rev().collect() 
// }

// fn reverse_loop(input: &str) -> String {
//     let mut s = String::new();
//     for c in input.chars().rev() {
//         s.push(c);
//     }
//     return s;
// }