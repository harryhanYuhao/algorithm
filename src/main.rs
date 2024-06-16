use algorithm::shuffled_vec_i64;

use algorithm::linear_algebra::Matrix2D;
use algorithm::sort::{insertion_sort, insertion_sort_inplace, merge_sort};

fn main() {
    let A = Matrix2D::identity(3);
    let B = Matrix2D::random(3, 3);
    println!("{:?}", B);
    // println!("{:?}", A.mul(&B));
    println!("{:?}", B.slice((0, 0), (2, 2)));
}
