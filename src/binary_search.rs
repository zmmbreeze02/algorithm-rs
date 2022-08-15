//! Binary search algorithm

/// This function implements the binary search algorithm
pub fn binary_search<T: Ord>(input: &[T], key: T) -> Option<usize> {
    let len = input.len();
    if len == 0 {
        return None;
    }

    let mut high = len - 1;
    let mut low: usize = 0;
    while low <= high {
        // left + (right - left) / 2 就和 (left + right) / 2 的结果相同
        // 但是有效防止了 left 和 right 太大直接相加导致溢出
        let middle = low + (high - low) / 2;
        let mid_value = &input[middle];
        if key == *mid_value {
            return Some(middle);
        } else if key > *mid_value {
            low = middle + 1;
        } else {
            high = middle - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn test_binary_search() {
        let input = [];
        assert_eq!(binary_search(&input, 0), None);

        let input = [0];
        assert_eq!(binary_search(&input, 0), Some(0));

        let input = [0, 1];
        assert_eq!(binary_search(&input, 0), Some(0));
        assert_eq!(binary_search(&input, 1), Some(1));

        let input = [0, 1, 2];
        assert_eq!(binary_search(&input, 0), Some(0));
        assert_eq!(binary_search(&input, 1), Some(1));
        assert_eq!(binary_search(&input, 2), Some(2));

        let input = [0, 1, 2, 3];
        assert_eq!(binary_search(&input, 0), Some(0));
        assert_eq!(binary_search(&input, 1), Some(1));
        assert_eq!(binary_search(&input, 2), Some(2));
        assert_eq!(binary_search(&input, 3), Some(3));

        let input = [0, 1, 2, 3, 4];
        assert_eq!(binary_search(&input, 0), Some(0));
        assert_eq!(binary_search(&input, 1), Some(1));
        assert_eq!(binary_search(&input, 2), Some(2));
        assert_eq!(binary_search(&input, 3), Some(3));
        assert_eq!(binary_search(&input, 4), Some(4));

        let input = [0, 1, 2, 3, 4, 5];
        assert_eq!(binary_search(&input, 0), Some(0));
        assert_eq!(binary_search(&input, 1), Some(1));
        assert_eq!(binary_search(&input, 2), Some(2));
        assert_eq!(binary_search(&input, 3), Some(3));
        assert_eq!(binary_search(&input, 4), Some(4));
        assert_eq!(binary_search(&input, 5), Some(5));


        let input = [0, 1, 2, 3, 4, 5, 6];
        assert_eq!(binary_search(&input, 0), Some(0));
        assert_eq!(binary_search(&input, 1), Some(1));
        assert_eq!(binary_search(&input, 2), Some(2));
        assert_eq!(binary_search(&input, 3), Some(3));
        assert_eq!(binary_search(&input, 4), Some(4));
        assert_eq!(binary_search(&input, 5), Some(5));
        assert_eq!(binary_search(&input, 6), Some(6));

        let input = [0, 1, 2, 2, 2, 2, 6];
        assert_eq!(binary_search(&input, 2), Some(3));

        let input = [0, 1, 2, 3, 3, 3];
        assert_eq!(binary_search(&input, 3), Some(4));
    }
}
