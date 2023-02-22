use std::fmt::Debug;

// Define a function called `bubble_sort` that takes a mutable slice of generic type `T`
// where `T` must implement the `PartialOrd` and `Debug` traits.
pub fn bubble_sort<T: PartialOrd + Debug> (v:&mut [T]) {

    // Loop through each position `p` in the slice `v`.
    for p in 0..v.len() {

        // Assume that the slice is already sorted.
        let mut sorted = true;

        // Loop through each pair of adjacent elements in the slice `v`, up to the 
        // position `v.len() - 1 - p` (which is where the largest element is expected to
        // be after each iteration of the outer loop).
        for i in 0..(v.len() -1 ) - p {

            // If the current element is greater than the next element, swap them.
            // Also set the `sorted` flag to `false` to indicate that the slice is not
            // yet fully sorted.
            if v[i] > v[i+1] {
                v.swap(i,i+1);
                sorted = false;
            }
        }

        // If the `sorted` flag is still `true` (i.e., the slice is already sorted), 
        // then exit the loop early.
        if sorted {
            return;
        }
    }
}

// Define some tests for the `bubble_sort` function.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test that `bubble_sort` can sort a vector of integers.
    fn test_bubble_sort() {
        let mut v = vec![4,6,1,8,11,13,3];
        bubble_sort(&mut v);
        assert_eq!(v,vec![1,3,4,6,8,11,13]);
    }
}
