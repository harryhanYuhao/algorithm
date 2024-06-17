pub fn heap_sort(input: &[i64]) -> Vec<i64> {
    fn get_left(index: usize) -> usize {
        (index + 1) * 2 - 1
    }

    fn get_right(index: usize) -> usize {
        (index + 1) * 2
    }

    // assuming that the tree on the right and left are already heap
    fn heapify(input: &mut [i64], index: usize) {
        let greater: i64;
        let greater_index: usize;

        if get_left(index) >= input.len() {
            return;
        } else if get_right(index) >= input.len() {
            greater = input[get_left(index)];
            greater_index = get_left(index);
        } else if input[get_left(index)] > input[get_right(index)] {
            greater = input[get_left(index)];
            greater_index = get_left(index);
        } else {
            greater = input[get_right(index)];
            greater_index = get_right(index);
        }

        // end of recursive condition
        if input[index] >= greater {
            return;
        }
        let tmp = input[index];
        input[greater_index] = tmp;
        input[index] = greater;
        heapify(input, greater_index);
    }

    let mut ret = input.to_vec();
    let length = ret.len();

    // heapify the whole vec
    for i in (0..=(length / 2 - 1)).rev() {
        heapify(&mut ret, i);
    }

    for i in 1..length {
        let max = ret[0];
        ret[0] = ret[length - i];
        ret[length - i] = max;
        heapify(&mut ret[0..length - i], 0);
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;
    #[test]
    fn test_heap_sort() {
        let length = 41232;
        let vals = shuffled_vec_i64(length);
        let sorted = ascending_vec_i64(length);
        assert_eq!(heap_sort(&vals), sorted);
    }
}
