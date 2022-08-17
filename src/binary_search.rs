//! Binary search algorithm

/// This function implements the binary search algorithm
/// 二分的基本思路是定义一个搜索区域，逐步收敛区域，区域内的值一定是在逼近搜索值
pub fn binary_search<T: Ord>(input: &[T], key: T) -> Option<usize> {
    let len = input.len();
    if len == 0 {
        return None;
    }

    // 搜索区域是 [0, len-1]
    let mut high = len - 1;
    let mut low: usize = 0;

    while low <= high {
        // `low + (high - low) / 2` 就和 `(low + high) / 2` 的结果相同
        // 但是有效防止了 low 和 high 太大直接相加导致溢出
        let middle = low + (high - low) / 2;
        let mid_value = &input[middle];

        if key == *mid_value {
            // 找到正确值
            return Some(middle);
        } else if key > *mid_value {
            // 搜索区域右移 [middle+1, high]
            low = middle + 1;
        } else {
            // 搜索区域左移 [low, middle-1]
            high = middle - 1;
        }
    }

    // 此时 low > high，故一定没找到值
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
