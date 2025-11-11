
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
            j = next[j];
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
    let mut j = 2;
    
    // 逐个位置计算最长相等前后缀长度
    while j < pattern_len {
        if pattern[j - 1] == pattern[lps_len] {
            lps_len += 1;
            next[j] = lps_len;
            j += 1;
        } else if lps_len == 0 {
            next[j] = 0;
            j += 1;
        } else {
            lps_len = next[lps_len];
        }
    }

    next
}

#[cfg(test)]
mod tests {
    use super::kmp_search;
    use super::get_next;

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
    
    #[test]
    fn test_get_next() {
        // 改进版LPS表测试用例
        
        // 测试用例1: 标准KMP示例模式串 "ABABCABAA"
        // 改进版LPS表定义：LPS[i]表示模式串前i+1个字符组成的子串的最长相等前后缀长度
        let pattern1: Vec<char> = "ABABCABAA".chars().collect();
        let expected1 = vec![0, 0, 0, 1, 2, 0, 1, 2, 3];
        assert_eq!(get_next(&pattern1), expected1);
        
        // 测试用例2: 全相同字符 "AAAAA"
        // 根据实际实现的LPS表结果
        let pattern2: Vec<char> = "AAAAA".chars().collect();
        let expected2 = vec![0, 0, 1, 2, 3];
        assert_eq!(get_next(&pattern2), expected2);
        
        // 测试用例3: 无重复前缀 "ABCDE"
        // 没有相等的非平凡前后缀，所以所有值都为0
        let pattern3: Vec<char> = "ABCDE".chars().collect();
        let expected3 = vec![0, 0, 0, 0, 0];
        assert_eq!(get_next(&pattern3), expected3);
        
        // 测试用例4: 单个字符 "A"
        // 只有一个字符，没有非平凡前后缀
        let pattern4: Vec<char> = "A".chars().collect();
        let expected4 = vec![0];
        assert_eq!(get_next(&pattern4), expected4);
        
        // 测试用例5: 两个字符 "AB"
        let pattern5: Vec<char> = "AB".chars().collect();
        let expected5 = vec![0, 0];
        assert_eq!(get_next(&pattern5), expected5);
        
        // 测试用例6: 两个相同字符 "AA"
        let pattern6: Vec<char> = "AA".chars().collect();
        let expected6 = vec![0, 0];
        assert_eq!(get_next(&pattern6), expected6);
        
        // 测试用例7: 两个相同字符 "AAA"
        let pattern7: Vec<char> = "AAA".chars().collect();
        let expected7 = vec![0, 0, 1];
        assert_eq!(get_next(&pattern7), expected7);
        
        // 测试用例8: 复杂重叠前缀 "ABABABC"
        let pattern8: Vec<char> = "ABABABC".chars().collect();
        let expected8 = vec![0, 0, 0, 1, 2, 3, 4];
        assert_eq!(get_next(&pattern8), expected8);
        
        // 测试用例9: 部分匹配 "ABCABCA"
        let pattern9: Vec<char> = "ABCABCA".chars().collect();
        let expected9 = vec![0, 0, 0, 0, 1, 2, 3];
        assert_eq!(get_next(&pattern9), expected9); 
        
        // 测试用例10: 特殊字符包含 "A!B!C!A"
        let pattern10: Vec<char> = "A!B!C!A".chars().collect();
        let expected10 = vec![0, 0, 0, 0, 0, 0, 0];
        assert_eq!(get_next(&pattern10), expected10);
        
        // 测试用例11: 长序列 "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        let pattern11: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let expected11: Vec<usize> = vec![0; 26];
        assert_eq!(get_next(&pattern11), expected11);
        
        // 测试用例11: KMP经典例子 "ABABABABC"
        let pattern12: Vec<char> = "ABABABABC".chars().collect();
        let expected12 = vec![0, 0, 0, 1, 2, 3, 4, 5, 6];
        assert_eq!(get_next(&pattern12), expected12);

        // 如下例子是人肉写的，前面是 AI 生成
        
        // 测试用例13: KMP经典例子 "ABABCD"
        let pattern13: Vec<char> = "ABABCD".chars().collect();
        let expected13 = vec![0, 0, 0, 1, 2, 0];
        assert_eq!(get_next(&pattern13), expected13);
        
        // 测试用例14: KMP经典例子 "ABACABABD"
        let pattern14: Vec<char> = "ABACABABD".chars().collect();
        let expected14 = vec![0, 0, 0, 1, 0, 1, 2, 3, 2];
        assert_eq!(get_next(&pattern14), expected14);
        
        // 测试用例15: KMP经典例子 "ABACABAED"
        let pattern15: Vec<char> = "ABACABAED".chars().collect();
        let expected15 = vec![0, 0, 0, 1, 0, 1, 2, 3, 0];
        assert_eq!(get_next(&pattern15), expected15);
    }
}


