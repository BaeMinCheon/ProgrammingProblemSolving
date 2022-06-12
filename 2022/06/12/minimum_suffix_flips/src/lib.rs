// https://leetcode.com/problems/minimum-suffix-flips/

struct Solution {}
impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut result = 0;
        let mut previous_bit = '0';
        for bit in target.chars() {
            if bit == previous_bit {
                // Do nothing
            } else {
                result += 1;
            }
            previous_bit = bit;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let target = String::from("10111");
        assert_eq!(Solution::min_flips(target), 3);
    }

    #[test]
    fn example_2() {
        let target = String::from("101");
        assert_eq!(Solution::min_flips(target), 3);
    }

    #[test]
    fn example_3() {
        let target = String::from("00000");
        assert_eq!(Solution::min_flips(target), 0);
    }
}