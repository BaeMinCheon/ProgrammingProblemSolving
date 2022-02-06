// https://leetcode.com/problems/palindrome-number/

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else {
            if x < 10 {
                true
            } else {
                let mut result = true;
                let mut digits: Vec<i32> = vec![];
                let mut number = x;
                while number > 0
                {
                    let digit = number % 10;
                    digits.push(digit);
                    number = number / 10;
                }
                let range = digits.len() / 2;
                for index in 0..range {
                    if digits[index] != digits[digits.len() - (index + 1)] {
                        result = false;
                        break
                    }
                }
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::is_palindrome(1001), true);
    }

    #[test]
    fn example_5() {
        assert_eq!(Solution::is_palindrome(12345), false);
    }
}