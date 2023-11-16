use std::collections::HashSet;
use std::vec::Vec;

// Returns a vector of all permutations of values within the set using a non-recurisve
// version of Heap's algorithm: https://en.wikipedia.org/wiki/Heap%27s_algorithm
pub fn heap_permutations(all_points: &HashSet<String>) -> Vec<Vec<&str>> {
    let mut ret: Vec<Vec<&str>> = Vec::new();
    let mut points: Vec<&str> = all_points.into_iter().map(|x| x.as_str()).collect();

    let size = all_points.len();
    let mut c: Vec<usize> = vec![0; size];

    ret.push(points.clone());

    let mut i: usize = 1;
    while i < size {
        let cc = c.get_mut(i).unwrap();
        if *cc < i {
            if i % 2 == 0 {
                points.swap(0, i);
            } else {
                points.swap(*cc, i);
            }
            ret.push(points.clone());
            *cc += 1;
            i = 1;
        } else {
            *cc = 0;
            i += 1;
        }
    }

    ret
}
