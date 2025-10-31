
/**
 * KMP (Knuth-Morris-Pratt) 字符串匹配算法
 * 
 * 该算法通过利用已匹配信息避免不必要的字符比较，提高字符串匹配效率
 * 时间复杂度：O(m+n)，其中m是模式串长度，n是文本串长度
 * 
 * @param text 文本串，要在其中查找模式串的字符串
 * @param pattern 模式串，要在文本串中查找的子串
 * @return 如果找到匹配，返回模式串在文本串中第一次出现的索引位置；否则返回None
 */
pub fn kmp_search(text: &str, pattern: &str) -> Option<usize> {
    // 将字符串转换为字符向量以便通过索引访问
    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();
    let text_len = text.len();
    let pattern_len = pattern.len();

    // 边界情况处理
    if pattern_len == 0 {
        return Some(0); // 空模式串总是匹配从索引0开始的位置
    }

    if text_len == 0 {
        return None; // 空文本串无法匹配非空模式串
    }

    if text_len < pattern_len {
        return None; // 文本串长度小于模式串，无法匹配
    }

    // 计算部分匹配表（next数组），记录模式串中前缀和后缀的最长公共部分长度
    let next = get_next(&pattern);
    
    // i: 文本串的当前位置
    // j: 模式串的当前位置
    let mut i = 0;
    let mut j = 0;

    // KMP算法的核心匹配循环
    while i < text_len && j < pattern_len {
        // 当前字符匹配成功，两个指针都向前移动
        if j == 0 || text[i] == pattern[j] {
            i += 1;
            j += 1;
        } else {
            // 当前字符匹配失败，模式串指针回退到合适位置继续匹配
            // 利用部分匹配表避免重复比较已知的字符
            j = next[j - 1];
        }
    }

    // 判断是否完全匹配
    if j >= pattern_len {
        return Some(i - pattern_len); // 返回匹配的起始位置
    }

    None // 未找到匹配
}

/**
 * 计算部分匹配表（Next数组），也称为 LPS (Longest Proper Prefix which is also Suffix)数组
 * 
 * 该表记录了模式串中每个位置的前缀和后缀的最长公共部分长度，用于在匹配失败时
 * 确定模式串指针应该回退到的位置，避免重复比较
 * 
 * @param pattern 模式串的字符向量
 * @return 部分匹配表，next[i]表示pattern[0..i]的最长相等前后缀长度
 */
fn get_next(pattern: &Vec<char>) -> Vec<usize> {
    let pattern_len = pattern.len();
    // 初始化部分匹配表，长度与模式串相同
    let mut next = vec![0; pattern_len];
    
    // lps_len: 当前已计算部分的最长相等前后缀长度
    let mut lps_len = 0;
    // i: 当前计算的位置索引
    let mut i = 1;
    
    // 逐个位置计算最长相等前后缀长度
    for i in 2..pattern.len() {
        if pattern[i] == pattern[lps_len] {
            lps_len += 1;
        } else if lps_len > 0 {
            while lps_len > 0 {
                lps_len = next[lps_len - 1]
            }
        } else {
            lps_len = 0;
        }

        next[i]= lps_len;
    }

    next
}

#[cfg(test)]
mod tests {
    use super::kmp_search;

    #[test]
    fn test_kmp_search() {
        // 基本匹配测试
        assert_eq!(kmp_search("hello world", "world"), Some(6));
        assert_eq!(kmp_search("ababcabcacbab", "abcac"), Some(5));
        
        // 边界情况测试
        assert_eq!(kmp_search("test", ""), Some(0)); // 空模式串
        assert_eq!(kmp_search("", "pattern"), None); // 空文本串
        assert_eq!(kmp_search("short", "longer pattern"), None); // 文本串比模式串短
        
        // 无匹配情况测试
        assert_eq!(kmp_search("abcdef", "xyz"), None);
        
        // 多次出现的情况，应返回第一次出现的位置
        assert_eq!(kmp_search("ababababa", "aba"), Some(0));
        
        // 重叠匹配测试
        assert_eq!(kmp_search("aaaaa", "aaa"), Some(0));
    }
}


