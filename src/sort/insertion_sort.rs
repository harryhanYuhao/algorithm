/// sort ascending
pub fn insertion_sort(input: &[i64]) -> Vec<i64> {
    let mut ret = input.to_vec();
    for i in 2..=ret.len() {
        let key = ret[i - 1];
        let mut j = i - 1;
        while j > 0 && ret[j - 1] > key {
            ret[j] = ret[j - 1];
            j -= 1;
        }
        ret[j] = key;
    }
    ret
}

pub fn insertion_sort_inplace(input: &mut [i64]) {
    for i in 2..=input.len() {
        let key = input[i - 1];
        let mut j = i - 1;
        while j > 0 && input[j - 1] > key {
            input[j] = input[j - 1];
            j -= 1;
        }
        input[j] = key;
    }
}

#[cfg(test)]
mod test {
    use super::insertion_sort;
    use crate::test::*;
    #[test]
    fn shuffled_vec() {
        test_shuffled_vec(insertion_sort);
    }

    #[test]
    fn small_random_vec() {
        test_random_vec(insertion_sort);
    }

    #[test]
    fn zero_vec() {
        test_zero_vec(insertion_sort);
    }

    #[test]
    fn one_vec() {
        test_one_vec(insertion_sort);
    }
}
