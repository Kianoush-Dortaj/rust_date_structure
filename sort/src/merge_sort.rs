use std::fmt::Debug;

// Define a generic function called `merge_sort` that takes a mutable vector `v` of type `T`.
// The `T` type parameter must implement the `PartialOrd` and `Debug` traits.
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // If the length of the vector `v` is less than or equal to 1, it is already sorted.
    // Return the vector as is.

    if v.len() <= 1 {
        return v;
    }

    // Create a new vector `res` with the same capacity as the original vector `v`.
    let mut res = Vec::with_capacity(v.len());

    // Split the vector `v` into two halves.
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    // Create iterators for the two halves of the vector `v`.
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    // Merge the two sorted halves of the vector `v` into one sorted vector.
    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        // If the next element in b is less than the next element in a, 
                        // push the next element in b onto the result vector
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();  // Get the next element from b
                    } else {
                        // Otherwise, push the next element in a onto the result vector
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();  // Get the next element from a
                    }
                }
                None => {
                    // If there are no more elements in b, push the remaining elements in a onto the result vector
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;  // Return the sorted vector
                } 
            },
            None => {
                // If there are no more elements in a, push the remaining elements in b onto the result vector
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;  // Return the sorted vector
            }
        }
    }
    
}

// Define some tests for the `merge_sort` function.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test that `merge_sort` can sort a vector of integers.
    fn test_merge_sort() {
        let v = vec![4,6,1,8,11,13,3];
        let v = merge_sort(v);
        assert_eq!(v,vec![1,3,4,6,8,11,13]);
    }
}
