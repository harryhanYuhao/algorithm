use super::*;
static MAX: u64 = u64::MAX;
static INTERVAL: u64 = 67280421310721;
static STEPS: u64 = MAX / INTERVAL - 1;

#[derive(Debug)]
struct Entry {
    key: u64,
    entry: u64,
}

pub struct Table {
    base: Vec<Vec<Entry>>,
}

pub fn new() -> Table {
    Table {
        base: (0..STEPS).map(|_| Vec::<Entry>::new()).collect(),
    }
}

fn hash(input: u64) -> u64 {
    const B: u64 = 2654435761;
    const C: u64 = 1597334677;

    let little_part: u32 = input as u32;
    let little_part = circular_shift_left_u32(little_part, 16) as u64 * B as u64;
    let greater_part: u32 = (input >> 32) as u32;
    let greater_part = circular_shift_right_u32(greater_part, 16);
    let greater_part = greater_part as u64 * C as u64;

    (little_part + greater_part) % STEPS
    // (b + input) % STEPS
}

impl Table {
    pub fn get(&self, key: u64) -> Option<u64> {
        let sub_vec = &self.base[hash(key) as usize];
        for i in sub_vec {
            if i.key == key {
                return Some(i.entry);
            }
        }
        None
    }

    pub fn insert(&mut self, key: u64, val: u64) -> Option<u64> {
        let sub_vec = &mut self.base[hash(key) as usize];
        // TODO: implement binary search
        for i in sub_vec.iter_mut() {
            if i.key == key {
                let old = i.entry;
                i.entry = val;
                return Some(old);
            }
        }

        // TODO: insert it in ascending order

        // the key is not registered in the map
        sub_vec.push(Entry { key, entry: val });
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_hash_function() {
        let interval = 100;
        let mut res: Vec<u64> = vec![0; (STEPS / interval + 1) as usize];
        for i in 111193..4137741 {
            res[(hash(i) / interval) as usize] += 1;
        }
        for i in 0..10 {
            println!("{}", res[i]);
        }
        for i in 1000..1010 {
            println!("{}", res[i]);
        }
        for i in 2000..2010 {
            println!("{}", res[i]);
        }
    }
    #[test]
    fn test_hash_table() {
        let mut tmp = new();
        let iter = 0..1000;
        for i in iter.clone() {
            let a = tmp.insert(i, 2 * i);
            assert_eq!(a, None);
        }
        for i in iter {
            assert_eq!(tmp.get(i), Some(2 * i));
        }

        tmp.insert(u64::MAX - 1, 15);
        // assert_eq!(tmp.get(u64::MAX - 1), Some(15));
    }
}
