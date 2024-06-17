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

pub fn seedded_shuffled_vec_i64(upper_bound: i64, seed:u64) -> Vec<i64> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut vals: Vec<i64> = (0..upper_bound).collect();
    vals.shuffle(&mut rng);
    vals
}
