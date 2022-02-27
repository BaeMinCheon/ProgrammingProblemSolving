// https://leetcode.com/problems/make-the-string-great/

struct Solution {}

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut string = String::from(s);
        if string.len() > 1 {
            let mut index = 0;
            while true {
                let size = string.len();
                if (index + 1) >= size {
                    break;
                } else {
                    let characters = string.as_bytes();
                    let character_a = characters[index] as i32;
                    let character_b = characters[index + 1] as i32;
                    let difference = character_a - character_b;
                    if difference.abs() == 32 {
                        string.remove(index);
                        string.remove(index);
                        if index > 0 {
                            index -= 1;
                        } else {
                            index = 0;
                        }
                    } else {
                        index += 1;
                    }
                }
            }
        }
        string
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::make_good(String::from("leEeetcode")), String::from("leetcode"));
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::make_good(String::from("abBAcC")), String::from(""));
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::make_good(String::from("s")), String::from("s"));
    }
}