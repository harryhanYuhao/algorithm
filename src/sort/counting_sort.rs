// assuming the inputs are greater than or equal to 0
// TODO: REMOVE THIS RESTRICTION
pub fn counting_sort(input: &[i64]) -> Vec<i64> {
    let copy = input.to_vec();
    let max = match copy.iter().max() {
        Some(a) => a,
        None => return copy,
    };
    let min = match copy.iter().min() {
        Some(a) => a,
        None => return copy,
    };

    let mut b: Vec<i64> = vec![0; copy.len()];
    let mut c: Vec<i64> = vec![0; *max as usize + 1];

    for i in copy.iter() {
        c[*i as usize] += 1;
    } // c[j]: number of entries equal to j in A

    for i in 1..c.len() {
        c[i] += c[i - 1];
    } // c[j]: number of entries smaller than or equal to j in A

    for i in 0..copy.len() {
        b[c[copy[i] as usize] as usize - 1] = copy[i];
        c[copy[i] as usize] -= 1;
    }

    b
}


#[cfg(test)]
mod test {
    use super::counting_sort;
    use crate::test::*;
    #[test]
    fn shuffled_vec() {
        test_shuffled_vec(counting_sort);
    }

    #[test]
    fn random_vec() {
        test_small_random_vec(counting_sort);
    }

    #[test]
    fn zero_vec() {
        test_zero_vec(counting_sort);
    }

    #[test]
    fn one_vec() {
        test_one_vec(counting_sort);
    }
}

