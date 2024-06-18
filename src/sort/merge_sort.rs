use crate::sort::insertion_sort;

pub fn merge_sort(input: &[i64]) -> Vec<i64> {
    // end of recursion
    if input.len() <= 4 {
        return insertion_sort(input);
    }

    let mut ret: Vec<i64> = Vec::new();
    let v1 = input[..input.len() / 2].to_vec();
    let v2 = input[input.len() / 2..].to_vec();
    let res1 = merge_sort(&v1);
    let res2 = merge_sort(&v2);
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < res1.len() || j < res2.len() {
        if i == res1.len() {
            ret.push(res2[j]);
            j += 1;
            continue;
        }
        if j == res2.len() {
            ret.push(res1[i]);
            i += 1;
            continue;
        }
        if res1[i] > res2[j] {
            ret.push(res2[j]);
            j += 1;
            continue;
        }
        ret.push(res1[i]);
        i += 1;
    }
    ret
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;
    #[test]
    fn shuffled_vec() {
        let length = 10;
        let seed = 34;
        assert_eq!(
            merge_sort(&seeded_shuffled_vec_i64(length, seed)),
            ascending_vec_i64(length)
        );

        let length = 2;
        assert_eq!(
            merge_sort(&seeded_shuffled_vec_i64(length, seed)),
            ascending_vec_i64(length)
        );

        let length = 3221;
        assert_eq!(
            merge_sort(&seeded_shuffled_vec_i64(length, seed)),
            ascending_vec_i64(length)
        );
    }

    #[test]
    fn random_vec() {
        let length = 324;
        let seed = 41;
        assert!(is_ascending(&merge_sort(&seeded_random_vec_i64(
            length, seed
        ))))
    }

    #[test]
    fn zero_vec() {
        assert_eq!(merge_sort(&Vec::new()), Vec::new());
    }

    #[test]
    fn one_vec() {
        assert_eq!(merge_sort(&vec![10]), vec![10])
    }
}
