use rand::prelude::*;
pub mod sort;

/// return a shuffled vector with values from 0 to upperbound-1 inclusively
pub fn shuffled_vec_i64(upper_bound: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut vals: Vec<i64> = (0..upper_bound).collect();
    vals.shuffle(&mut rng);
    vals
}
