use std::collections::BTreeMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    grade_namelist: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            grade_namelist: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grade_namelist
            .entry(grade)
            .or_insert(Vec::new())
            .push(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        // let mut list = self.grade_namelist.keys()
        //     .map(|grade| *grade)
        //     .collect::<Vec<u32>>();
        // list.sort();
        // list

        self.grade_namelist.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut namelist = self
            .grade_namelist
            .get(&grade)
            .unwrap_or(&Vec::new())
            .to_vec();
        namelist.sort();
        namelist
    }
}
