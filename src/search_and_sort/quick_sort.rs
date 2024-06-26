pub fn quick_sort(input: &[i64]) -> Vec<i64> {
    let mut ret = input.to_vec();
    let length = ret.len();
    if length <= 1 {
        return ret;
    }
    quick_sort_inplace(&mut ret, 0, length - 1);
    ret
}

fn partition(input: &mut [i64], start: usize, end: usize) -> usize {
    let pivot = input[end];
    let mut i = start as i64 - 1;
    for j in start..end {
        if input[j] <= pivot {
            i += 1;
            input.swap(i as usize, j)
        }
    }
    input.swap((i + 1) as usize, end);
    (i + 1) as usize
}

fn quick_sort_inplace(input: &mut [i64], start: usize, end: usize) {
    if start >= end {
        return;
    }

    let q = partition(input, start, end);
    if q != 0 {
        quick_sort_inplace(input, start, q - 1);
    }
    quick_sort_inplace(input, q + 1, end);
}

#[cfg(test)]
mod test {
    use super::quick_sort;
    use crate::test::*;
    #[test]
    fn shuffled_vec() {
        test_shuffled_vec(quick_sort);
    }

    #[test]
    fn random_vec() {
        test_small_random_vec(quick_sort);
    }

    #[test]
    fn zero_vec() {
        test_zero_vec(quick_sort);
    }

    #[test]
    fn one_vec() {
        test_one_vec(quick_sort);
    }

    #[test]
    fn two_vec() {
        test_two_vec(quick_sort);
    }

    #[test]
    fn three_vec() {
        test_three_vec(quick_sort);
    }
}
