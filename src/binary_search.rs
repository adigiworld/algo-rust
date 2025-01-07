// type search = PartialEq + PartialOrd;
// pub fn binary_search<T: PartialEq + std::cmp::PartialOrd>(haystack: &Vec<T>, needle: T) -> bool {
pub fn binary_search<T>(haystack: &Vec<T>, needle: T) -> bool
where
    T: PartialEq + std::cmp::PartialOrd,
{
    let mut low: usize = 0;
    let mut hig: usize = haystack.len();
    while low < hig {
        let mid = low + (hig - low) / 2;
        if haystack[mid] == needle {
            return true;
        } else if needle < haystack[mid] {
            hig = mid;
        } else {
            low = mid + 1;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alg_binary_search() {
        let haystack: Vec<u64> = vec![
            1, 3, 4, 25, 37, 69, 71, 81, 90, 99, 420, 1225, 2568, 12365, 63421,
        ];
        assert_eq!(binary_search(&haystack, 1), true);
        assert_eq!(binary_search(&haystack, 13), false);
        assert_eq!(binary_search(&haystack, 81), true);
        assert_eq!(binary_search(&haystack, 12256), false);
        assert_eq!(binary_search(&haystack, 63421), true);
        assert_eq!(binary_search(&haystack, 64000), false);
    }
}
