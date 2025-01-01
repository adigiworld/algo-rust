pub fn linear_search(haystack: &Vec<u64>, needle: u64) -> bool {
    for n in haystack {
        if *n == needle {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{u64, vec};

    #[test]
    fn alg_linear_search() {
        let haystack: Vec<u64> = vec![
            1, 3, 4, 25, 37, 69, 71, 81, 90, 99, 420, 1225, 2568, 12365, 63421,
        ];
        assert_eq!(linear_search(&haystack, 1), true);
        assert_eq!(linear_search(&haystack, 13), false);
        assert_eq!(linear_search(&haystack, 81), true);
        assert_eq!(linear_search(&haystack, 12256), false);
        assert_eq!(linear_search(&haystack, 63421), true);
    }
}
