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
