// https://leetcode.com/problems/number-of-1-bits/

struct Solution {}

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut result = 0;
        let mut number = n;
        while number > 0 {
            if (number & 0b1) == 1 {
                result += 1;
            }
            number /= 2;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::hammingWeight(0b00000000000000000000000000001011), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::hammingWeight(0b00000000000000000000000010000000), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::hammingWeight(0b11111111111111111111111111111101), 31);
    }
}