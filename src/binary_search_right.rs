//! Binary search algorithm

/// This function implements the binary search algorithm
/// 二分的基本思路是定义一个搜索区域，逐步收敛区域，区域内的值一定是在逼近搜索值
/// 本题只是如果出现多个满足要求的值，则逼近这个区域的右边界
pub fn binary_search_right<T: Ord>(input: &[T], key: T) -> Option<usize> {
    let len = input.len();
    if len == 0 {
        return None;
    }
    if len == 1 {
        return if input[0] == key { Some(0) } else { None }
    }

    // 搜索区间是 [0, len-1]
    let mut low = 0;
    let mut high = len - 1;
    while low < high {
        let middle = low + (high - low) / 2;
        let mid_value = &input[middle];

        if key == *mid_value {
            // 搜索区间右移 [middle+1, high]
            // 可能会导致正确值被略过
            low = middle + 1;
        } else if key > *mid_value {
            // 搜索区间右移 [middle+1, high]
            low = middle + 1;
        } else {
            // 搜索区间左移 [low, middle-1]
            high = middle - 1;
        }
    }

    // 此时 low == high，搜索区间内只有一个值
    // 如果值是要搜索的值，则返回low，否则为搜索不到
    if low == 0 || key != input[low - 1] { None } else { Some(low - 1) }
}



#[cfg(test)]
mod tests {
    use super::binary_search_right;

    #[test]
    fn test_binary_search_right() {
        // let input = [];
        // assert_eq!(binary_search_right(&input, 0), None);

        // let input = [0];
        // assert_eq!(binary_search_right(&input, 0), Some(0));

        let input = [0, 1];
        // assert_eq!(binary_search_right(&input, 0), Some(0));
        assert_eq!(binary_search_right(&input, 1), Some(1));

        // let input = [0, 1, 2];
        // assert_eq!(binary_search_right(&input, 0), Some(0));
        // assert_eq!(binary_search_right(&input, 1), Some(1));
        // assert_eq!(binary_search_right(&input, 2), Some(2));

        // let input = [0, 1, 2, 3];
        // assert_eq!(binary_search_right(&input, 0), Some(0));
        // assert_eq!(binary_search_right(&input, 1), Some(1));
        // assert_eq!(binary_search_right(&input, 2), Some(2));
        // assert_eq!(binary_search_right(&input, 3), Some(3));

        // let input = [0, 1, 2, 3, 4];
        // assert_eq!(binary_search_right(&input, 0), Some(0));
        // assert_eq!(binary_search_right(&input, 1), Some(1));
        // assert_eq!(binary_search_right(&input, 2), Some(2));
        // assert_eq!(binary_search_right(&input, 3), Some(3));
        // assert_eq!(binary_search_right(&input, 4), Some(4));

        // let input = [0, 1, 2, 3, 4, 5];
        // assert_eq!(binary_search_right(&input, 0), Some(0));
        // assert_eq!(binary_search_right(&input, 1), Some(1));
        // assert_eq!(binary_search_right(&input, 2), Some(2));
        // assert_eq!(binary_search_right(&input, 3), Some(3));
        // assert_eq!(binary_search_right(&input, 4), Some(4));
        // assert_eq!(binary_search_right(&input, 5), Some(5));


        // let input = [0, 1, 2, 3, 4, 5, 6];
        // assert_eq!(binary_search_right(&input, 0), Some(0));
        // assert_eq!(binary_search_right(&input, 1), Some(1));
        // assert_eq!(binary_search_right(&input, 2), Some(2));
        // assert_eq!(binary_search_right(&input, 3), Some(3));
        // assert_eq!(binary_search_right(&input, 4), Some(4));
        // assert_eq!(binary_search_right(&input, 5), Some(5));
        // assert_eq!(binary_search_right(&input, 6), Some(6));

        // let input = [2];
        // assert_eq!(binary_search_right(&input, 2), Some(0));

        // let input = [2, 2, 3, 6];
        // assert_eq!(binary_search_right(&input, 2), Some(1));

        // let input = [0, 2, 2, 3, 6];
        // assert_eq!(binary_search_right(&input, 2), Some(2));

        // let input = [0, 1, 2, 2, 3, 3, 6];
        // assert_eq!(binary_search_right(&input, 2), Some(3));

        // let input = [0, 1, 2, 2, 2, 3, 3, 3, 6];
        // assert_eq!(binary_search_right(&input, 2), Some(4));

        // let input = [0, 1, 2, 2, 2, 2, 3, 3, 3, 6];
        // assert_eq!(binary_search_right(&input, 2), Some(5));

        // let input = [0, 1, 1, 1, 2];
        // assert_eq!(binary_search_right(&input, 2), Some(4));

        // let input = [0, 1, 1, 1, 1, 2];
        // assert_eq!(binary_search_right(&input, 2), Some(5));
    }
}
