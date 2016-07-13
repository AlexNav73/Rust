
use std::cmp::Ordering;

struct JaggetArray<T: Ord>(Vec<Vec<T>>);

impl<T: Ord> JaggetArray<T> {

    fn new(v: Vec<Vec<T>>) -> JaggetArray<T> {
        JaggetArray(v)
    }

    fn sort<F>(&mut self, cmp: F) where F: Fn(&Vec<T>, &Vec<T>) -> Ordering {
        let len = self.0.len();

        for i in 0..len {
            let mut swapped = false;
            
            for j in 0..(len - i - 1) {
                match cmp(&self.0[j], &self.0[j + 1]) {
                    Ordering::Greater | Ordering::Equal => { },
                    Ordering::Less => {
                        self.0.as_mut_slice().swap(j, j + 1);
                        swapped = true;
                    }
                }
            }

            if !swapped { break; }
        }
    }

}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::JaggetArray;

    #[test]
    fn test_jagged_array_sum() {
        let mut v = JaggetArray::new(vec![vec![3, 3, 3], vec![1, 1, 1], vec![2, 2, 2]]);
        v.sort(sum);

        assert_eq!(v.0, vec![vec![3, 3, 3], vec![2, 2, 2], vec![1, 1, 1]]);
    }

    #[test]
    fn test_jagged_array_max() {
        let mut v = JaggetArray::new(vec![vec![3, 3, 3], vec![1, 1, 1], vec![2, 2, 2]]);
        v.sort(max);

        assert_eq!(v.0, vec![vec![3, 3, 3], vec![2, 2, 2], vec![1, 1, 1]]);
    }

    #[test]
    fn test_jagged_array_min() {
        let mut v = JaggetArray::new(vec![vec![3, 3, 3], vec![1, 1, 1], vec![2, 2, 2]]);
        v.sort(min);

        assert_eq!(v.0, vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3]]);
    }
    
    fn sum(v1: &Vec<i32>, v2: &Vec<i32>) -> Ordering {
        let sum1 = v1.iter().fold(0i32, |acc, x| x + acc);
        let sum2 = v2.iter().fold(0i32, |acc, x| x + acc);
        sum1.cmp(&sum2)
    }

    fn max(v1: &Vec<i32>, v2: &Vec<i32>) -> Ordering {
        let max1 = v1.iter().max().unwrap();
        let max2 = v2.iter().max().unwrap();
        max1.cmp(&max2)
    }

    fn min(v1: &Vec<i32>, v2: &Vec<i32>) -> Ordering {
        let min1 = v1.iter().min().unwrap();
        let min2 = v2.iter().min().unwrap();
        min2.cmp(&min1)
    }

}

