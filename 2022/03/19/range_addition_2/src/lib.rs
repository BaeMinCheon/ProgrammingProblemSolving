// https://leetcode.com/problems/range-addition-ii/

struct Solution {}

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut result = m * n;
        if ops.len() > 0 {
            let mut min_size_row = m;
            let mut min_size_col = n;
            for op in ops {
                let row = op.get(0).unwrap().clone();
                if row < min_size_row {
                    min_size_row = row;
                }
                let col = op.get(1).unwrap().clone();
                if col < min_size_col {
                    min_size_col = col;
                }
            }
            result = min_size_row * min_size_col;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let ops = vec![vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::max_count(3, 3, ops), 4);
    }

    #[test]
    fn example_2() {
        let ops = vec![vec![2, 2],vec![3, 3],vec![3, 3],vec![3, 3],vec![2, 2],vec![3, 3],vec![3, 3],vec![3, 3],vec![2, 2],vec![3, 3],vec![3, 3],vec![3, 3]];
        assert_eq!(Solution::max_count(3, 3, ops), 4);
    }

    #[test]
    fn example_3() {
        let ops = vec![];
        assert_eq!(Solution::max_count(3, 3, ops), 9);
    }

    #[test]
    fn example_4() {
        let ops = vec![vec![1, 2], vec![2, 3]];
        assert_eq!(Solution::max_count(2, 3, ops), 2);
    }
}