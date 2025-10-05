// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;

    // Previously, .to_vec() was needed because thread::spawn requires a 'static lifetime. See: exercises/07_threads/01_threads/src/lib.rs
    // to_vec() clones the data so each thread owns its own copy and can be safely moved into the thread.

    // Now, with thread::scope, you can safely borrow the data without cloning it.
    // Because thread::scope guarantees that all spawned threads finish before the scope ends,
    // the borrowed data (like slices of v) will still be valid for the entire lifetime of those threads — no need to clone.
    thread::scope(|s| {
        let (left, right) = v.split_at(mid);
        let left_sum = s.spawn(move || left.iter().sum::<i32>());
        let right_sum = s.spawn(move || right.iter().sum::<i32>());
        left_sum.join().unwrap() + right_sum.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
