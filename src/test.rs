use rand::prelude::*;

pub fn ascending_vec_i64(upper_bound: i64) -> Vec<i64> {
    (0..upper_bound).collect()
}

/// return a shuffled vector with values from 0 to upperbound-1 inclusively
pub fn shuffled_vec_i64(upper_bound: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut vals: Vec<i64> = (0..upper_bound).collect();
    vals.shuffle(&mut rng);
    vals
}

pub fn seeded_shuffled_vec_i64(upper_bound: i64, seed: u64) -> Vec<i64> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut vals: Vec<i64> = (0..upper_bound).collect();
    vals.shuffle(&mut rng);
    vals
}

pub fn seeded_random_vec_i64(length: usize, seed: u64) -> Vec<i64> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut ret: Vec<i64> = vec![0; length];
    for i in ret.iter_mut() {
        *i = rng.gen()
    }
    ret
}

pub fn seeded_small_random_vec_i64(length: usize, seed: u64) -> Vec<i64> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut ret = vec![0; length];
    for i in ret.iter_mut() {
        *i = rng.gen_range(-2_i64.pow(12)..2_i64.pow(12));
    }
    ret
}

pub fn is_ascending(input: &[i64]) -> bool {
    for i in 1..input.len() {
        if input[i - 1] > input[i] {
            return false;
        }
    }
    true
}

pub fn test_shuffled_vec(ptr: fn(&[i64]) -> Vec<i64>) {
    let length = 10;
    let seed = 34;
    assert!(is_ascending(&ptr(&seeded_shuffled_vec_i64(length, seed))));
    let length = 2;
    assert!(is_ascending(&ptr(&seeded_shuffled_vec_i64(length, seed))));
    let length = 3221;
    assert!(is_ascending(&ptr(&seeded_shuffled_vec_i64(length, seed))));
}

pub fn test_random_vec(ptr: fn(&[i64]) -> Vec<i64>) {
    let length = 324;
    let seed = 41;
    assert!(is_ascending(&ptr(&seeded_random_vec_i64(length, seed))))
}

pub fn test_small_random_vec(ptr: fn(&[i64]) -> Vec<i64>) {
    let length = 324;
    let seed = 41;
    assert!(is_ascending(&ptr(&seeded_small_random_vec_i64(length, seed))))
}

pub fn test_zero_vec(ptr: fn(&[i64]) -> Vec<i64>) {
    assert_eq!(ptr(&Vec::new()), Vec::new());
}

pub fn test_one_vec(ptr: fn(&[i64]) -> Vec<i64>) {
    assert_eq!(ptr(&vec![10]), vec![10])
}
