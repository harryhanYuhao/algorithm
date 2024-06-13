use algorithm::shuffled_vec_i64;

use algorithm::sort::{insertion_sort, insertion_sort_inplace, merge_sort};

fn main() {
    let mut vals = shuffled_vec_i64(100000);
    println!("{:?}", vals);
    let mut res: Vec<i64> = Vec::new();
    res = merge_sort(&mut vals);
    println!("{:?}", res);
    // insertion_sort_inplace(&mut vals);
    // println!("{:?}", vals);
    // println!("DEBUGGING:");
    // let tmp: Vec<i64> = (0..100).collect();
    // println!("{:?}", tmp[..10].to_vec());
    // println!("{:?}", tmp[10..].to_vec());
}
