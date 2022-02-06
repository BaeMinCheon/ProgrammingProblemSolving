// https://leetcode.com/problems/latest-time-by-replacing-hidden-digits/

struct Solution {}

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let vec: Vec<char> = time.chars().collect();
        let mut hour = 0;
        if vec[0] == '?' {
            hour += 20;
        } else {
            hour += vec[0].to_digit(10).unwrap() * 10;
        }
        if vec[1] == '?' {
            hour += 9;
        } else {
            hour += vec[1].to_digit(10).unwrap();
        }
        if hour > 23 {
            if vec[1] == '?' {
                hour -= 6;
            } else if vec[0] == '?' {
                hour -= 10;
            }
        }
        let mut minute = 0;
        if vec[3] == '?' {
            minute += 50;
        } else {
            minute += vec[3].to_digit(10).unwrap() * 10;
        }
        if vec[4] == '?' {
            minute += 9;
        } else {
            minute += vec[4].to_digit(10).unwrap();
        }
        format!("{:02}:{:02}", hour, minute)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::maximum_time(String::from("2?:?0")), String::from("23:50"));
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::maximum_time(String::from("0?:3?")), String::from("09:39"));
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::maximum_time(String::from("1?:22")), String::from("19:22"));
    }
}