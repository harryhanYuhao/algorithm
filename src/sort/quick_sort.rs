pub fn quick_sort(input: &[i64]) -> Vec<i64> {
    let mut ret = input.to_vec();
    let length = ret.len();
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
    use crate::{sort::quick_sort, test::*};
    #[test]
    fn test_quick_sort() {
        let length = 10000;
        let vals = shuffled_vec_i64(length);
        let target = ascending_vec_i64(length);
        let sorted = quick_sort(&vals);
        assert_eq!(sorted, target);
    }
}
