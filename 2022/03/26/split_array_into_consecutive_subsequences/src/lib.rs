// https://leetcode.com/problems/split-array-into-consecutive-subsequences/
// Inspired by https://leetcode.com/problems/split-array-into-consecutive-subsequences/discuss/836253/Rust-translated-20ms-100

struct Solution {}

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        if nums.len() >= 3 {
            let mut result = true;
            let mut num_count: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
            for &num in &nums {
                *(num_count.entry(num).or_default()) += 1;
            }
            let mut imaginary_count: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
            for &num in &nums {
                if *(num_count.get(&num).unwrap()) > 0 {
                    // Branch #1. Check there is a Consecutive Increasing Sequence 
                    if *(imaginary_count.get(&num).or(Some(&0)).unwrap()) > 0 {
                        // Reduce count
                        *(num_count.entry(num).or_default()) -= 1;
                        {   // Reduce count
                            *(imaginary_count.entry(num).or_default()) -= 1;
                            // Suppose there is number. We will visit Branch #1 if there is (num + 1) in nums, otherwise this line will be meaningless.
                            *(imaginary_count.entry(num + 1).or_default()) += 1;
                        }
                    } else
                    // Branch #2. When (num + 1) and (num + 2) exist in nums
                    if (*(num_count.get(&(num + 1)).or(Some(&0)).unwrap()) > 0) && (*(num_count.get(&(num + 2)).or(Some(&0)).unwrap()) > 0)
                    {
                        {   // Split a subsequence
                            *(num_count.entry(num + 0).or_default()) -= 1;
                            *(num_count.entry(num + 1).or_default()) -= 1;
                            *(num_count.entry(num + 2).or_default()) -= 1;
                        }
                        // Suppose there is number. We will visit Branch #1 if there is (num + 3) in nums, otherwise this line will be meaningless.
                        *(imaginary_count.entry(num + 3).or_default()) += 1;
                    } else
                    // Branch #3. We still have left numbers in nums, but cannot make any further subsequence.
                    {
                        result = false;
                        break;
                    }
                }
            }
            result
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3, 3, 4, 5];
        assert_eq!(Solution::is_possible(nums), true);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 2, 3, 3, 4, 4, 5, 5];
        assert_eq!(Solution::is_possible(nums), true);
    }

    #[test]
    fn example_3() {
        let nums = vec![1, 2, 3, 4, 4, 5];
        assert_eq!(Solution::is_possible(nums), false);
    }

    #[test]
    fn example_4() {
        let nums = vec![1, 2, 3, 5, 5, 6, 7];
        assert_eq!(Solution::is_possible(nums), false);
    }
}