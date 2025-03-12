pub fn merge_sort<T>(list: &mut Vec<T>)
where
    T: PartialEq + PartialOrd + Copy,
{
    sort(list, 0, list.len() - 1);
}

fn sort<T>(arr: &mut Vec<T>, low: usize, hig: usize)
where
    T: PartialEq + PartialOrd + Copy,
{
    if low >= hig {
        return;
    }
    let mid_idx: usize = low + (hig - low) / 2;
    sort(arr, low, mid_idx);
    sort(arr, mid_idx + 1, hig);

    merge(arr, low, mid_idx, hig);
}

fn merge<T>(arr: &mut Vec<T>, low: usize, mid: usize, hig: usize)
where
    T: PartialEq + PartialOrd + Copy,
{
    let left_len = (mid - low) + 1;
    let right_len = hig - mid;
    let mut sort_left: Vec<T> = vec![];
    let mut sort_right: Vec<T> = vec![];
    for i in 0..left_len {
        sort_left.push(arr[low + i]);
    }
    for j in 0..right_len {
        sort_right.push(arr[mid + 1 + j]);
    }
    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut k: usize = low;
    while l < left_len && r < right_len {
        if sort_left[l] < sort_right[r] {
            arr[k] = sort_left[l];
            l += 1;
        } else {
            arr[k] = sort_right[r];
            r += 1;
        }
        k += 1;
    }
    while l < left_len {
        arr[k] = sort_left[l];
        l += 1;
        k += 1;
    }
    while r < right_len {
        arr[k] = sort_right[r];
        r += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alg_merge_sort() {
        let mut arr: Vec<i64> = vec![9, 3, 7, 4, 69, 420, 42, 125, 245];
        merge_sort(&mut arr);
        assert_eq!(arr, [3, 4, 7, 9, 42, 69, 125, 245, 420]);
        //println!("{:?}", arr);

        let mut array: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        merge_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
