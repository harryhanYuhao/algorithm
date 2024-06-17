use algorithm::shuffled_vec_i64;

use algorithm::linear_algebra::Matrix2D;
use algorithm::sort::{heap_sort, insertion_sort, insertion_sort_inplace, merge_sort};

fn main() {
    let random_vec = shuffled_vec_i64(10000);
    println!("{:?}", random_vec);
    let ret = merge_sort(&random_vec);
    println!("{:?}", ret);
}
