// https://leetcode.com/problems/number-of-1-bits/

struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let mut result = false;
        if n >= 1 {
            let mut number = n;
            while number > 1 {
                if (number % 4) != 0 {
                    break;
                } else {
                    number /= 4;
                }
            }
            if number == 1 {
                result = true;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_power_of_four(16), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::is_power_of_four(5), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::is_power_of_four(1), true);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::is_power_of_four(8), false);
    }
}