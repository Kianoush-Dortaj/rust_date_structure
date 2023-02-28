pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = pivot(arr);
    let pivot = arr[pivot_index];
    arr.swap(pivot_index, arr.len() - 1);

    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut i = 0;

    while i <= right {
        if arr[i] < pivot {
            arr.swap(i, left);
            left += 1;
            i += 1;
        } else if arr[i] > pivot {
            arr.swap(i, right);
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            i += 1;
        }
    }

    quick_sort(&mut arr[..left]);
    quick_sort(&mut arr[right + 1..]);
}

fn pivot<T: Ord>(arr: &[T]) -> usize {
    if arr.len() <= 1 {
        return 0;
    }
    let mid = arr.len() / 2;
    if arr[0] <= arr[mid] && arr[mid] <= arr[arr.len() - 1] {
        mid
    } else if arr[mid] <= arr[0] && arr[0] <= arr[arr.len() - 1] {
        0
    } else {
        arr.len() - 1
    }
}
