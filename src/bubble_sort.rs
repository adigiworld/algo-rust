pub fn bubble_sort<T>(list: &mut Vec<T>)
where
    T: PartialEq + PartialOrd + Copy,
{
    let mut swapped: bool;
    // let mut count = 0;
    for i in 0..list.len() {
        swapped = false;
        for j in 0..list.len() - 1 - i {
            if list[j] > list[j + 1] {
                let temp = list[j + 1];
                list[j + 1] = list[j];
                list[j] = temp;
                swapped = true;
            }
            // count += 1;
        }
        if !swapped {
            break;
        }
    }
    // println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alg_bubble_sort() {
        let mut arr: Vec<i64> = vec![9, 3, 7, 4, 69, 420, 42, 125, 245];
        bubble_sort(&mut arr);
        assert_eq!(arr, [3, 4, 7, 9, 42, 69, 125, 245, 420]);
        // println!("{:?}", arr);

        let mut array: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        bubble_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
