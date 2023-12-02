use std::collections::HashSet;
use std::vec::Vec;

/// Returns a vector of all permutations of values within the set using a non-recurisve
/// version of Heap's algorithm: https://en.wikipedia.org/wiki/Heap%27s_algorithm
pub fn heap_permutations(all_elements: &HashSet<String>) -> Vec<Vec<&str>> {
    let mut ret: Vec<Vec<&str>> = Vec::new();
    let mut elements: Vec<&str> = all_elements.into_iter().map(|x| x.as_str()).collect();

    let size = all_elements.len();
    let mut c: Vec<usize> = vec![0; size];

    ret.push(elements.clone());

    let mut i: usize = 1;
    while i < size {
        let cc = c.get_mut(i).unwrap();
        if *cc < i {
            if i % 2 == 0 {
                elements.swap(0, i);
            } else {
                elements.swap(*cc, i);
            }
            ret.push(elements.clone());
            *cc += 1;
            i = 1;
        } else {
            *cc = 0;
            i += 1;
        }
    }

    ret
}
