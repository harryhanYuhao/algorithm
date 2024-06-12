use algorithm::shuffled_vec_i64;

use algorithm::sort::{insertion_sort, merge_sort};

fn main() {
    let vals = shuffled_vec_i64(10000);
    println!("{:?}", vals);
    for i in 0..10 {
        let res = merge_sort(&vals);
    }
    // println!("{:?}", res);

    // println!("DEBUGGING:");
    // let tmp: Vec<i64> = (0..100).collect();
    // println!("{:?}", tmp[..10].to_vec());
    // println!("{:?}", tmp[10..].to_vec());
}
