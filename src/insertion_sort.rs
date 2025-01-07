pub fn insertion_sort<T>(list: &mut Vec<T>)
where
    T: PartialEq + PartialOrd + Copy,
{
    // let mut count = 0;
    for i in 1..list.len() {
        let mut swapped = false;
        let mut j = i;
        while j > 0 {
            // count += 1;
            if list[j] < list[j - 1] {
                let temp = list[j - 1];
                list[j - 1] = list[j];
                list[j] = temp;
                swapped = true;
            }
            if !swapped {
                break;
            }
            j -= 1;
        }
    }
    // println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alg_insertion_sort() {
        let mut arr: Vec<i64> = vec![9, 3, 7, 4, 69, 420, 42, 125, 245];
        insertion_sort(&mut arr);
        assert_eq!(arr, [3, 4, 7, 9, 42, 69, 125, 245, 420]);
        // println!("{:?}", arr);

        let mut array: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        insertion_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
