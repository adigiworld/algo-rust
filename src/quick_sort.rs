pub fn quick_sort<T>(list: &mut Vec<T>)
where
    T: PartialEq + PartialOrd + Copy,
{
    q_s(list, 0, list.len() - 1);
}

fn q_s<T>(arr: &mut Vec<T>, low: usize, hig: usize)
where
    T: PartialEq + PartialOrd + Copy,
{
    if low >= hig {
        return;
    }
    let pivot_idx: usize = partition(arr, low, hig);
    q_s(arr, low, pivot_idx - 1);
    q_s(arr, pivot_idx + 1, hig);
}

fn partition<T>(arr: &mut Vec<T>, low: usize, hig: usize) -> usize
where
    T: PartialEq + PartialOrd + Copy,
{
    let pivot = arr[hig];
    let mut idx: usize = low;
    let mut i = low;
    while i < hig {
        if arr[i] <= pivot {
            let temp = arr[i];
            arr[i] = arr[idx];
            arr[idx] = temp;
            idx += 1;
        }
        i += 1;
    }
    // idx += 1;
    arr[hig] = arr[idx];
    arr[idx] = pivot;
    return idx;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alg_quick_sprt() {
        let mut arr: Vec<i64> = vec![9, 3, 7, 4, 69, 420, 42, 125, 245];
        quick_sort(&mut arr);
        assert_eq!(arr, [3, 4, 7, 9, 42, 69, 125, 245, 420]);
        println!("{:?}", arr);

        let mut array: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        quick_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
