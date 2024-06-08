use algorithm::shuffled_vec_i64;

fn main() {
    let mut vals = shuffled_vec_i64(100);
    println!("{:?}", vals);
    for i in 2..vals.len() {
        let key = vals[i-1];
        let mut j = i - 1;
        while j > 0  && vals[j-1] > key {
            vals[j] = vals[j-1];
            j -= 1; 
        }
        vals[j] = key;
    }
    println!("{:?}", vals);
}
