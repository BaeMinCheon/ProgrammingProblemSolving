// https://leetcode.com/problems/prime-number-of-set-bits-in-binary-representation/

struct Solution {}

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut result = 0;
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
        for number in left..=right {
            let mut remain = number;
            let mut count = 0;
            while remain > 0 {
                if remain & 1 == 1 {
                    count += 1;
                }
                remain /= 2;
            }
            if primes.contains(&count) {
                result += 1;
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
        assert_eq!(Solution::count_prime_set_bits(6, 10), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::count_prime_set_bits(10, 15), 5);
    }
}