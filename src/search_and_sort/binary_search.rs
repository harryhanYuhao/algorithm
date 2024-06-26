///expect input in ascending order
pub fn binary_search<T>(vec: &[T], expected: T) -> Option<usize>
where
    T: PartialEq + PartialOrd
{
    let mut low = 0;
    let mut high = vec.len(); // not vec.len() - 1
    while low < high {
        let index = (low + high) / 2;
        if vec[index] == expected {
            return Some(index);
        } else if vec[index] < expected {
            low = index;
        } else {
            high = index;
        }
    }

    return None
}

#[cfg(test)]
mod test {
    use super::binary_search;

    #[test]
    fn i64_vec() {
    }

    #[test]
    fn empty_vec() {
        let vec = Vec::new();
        let res = binary_search(&vec, 0);
        assert_eq!(res, None);
    }

    #[test]
    fn one_vec(){
        let vec = vec![1];
        let res = binary_search(&vec, 1);
        assert_eq!(res, Some(0));
    }

    #[test]
    fn two_vec(){
        let vec = vec![1, 2];
        let res = binary_search(&vec, 1);
        assert_eq!(res, Some(0));

        let res = binary_search(&vec, 2);
        assert_eq!(res, Some(1));
    }

    #[test]
    fn three_vec(){
        let vec = vec![1, 2, 3];
        let res = binary_search(&vec, 1);
        assert_eq!(res, Some(0));

        let res = binary_search(&vec, 2);
        assert_eq!(res, Some(1));

        let res = binary_search(&vec, 3);
        assert_eq!(res, Some(2));
    }

    #[test]
    fn four_vec(){
        let vec = vec![1, 2, 3, 4];
        let res = binary_search(&vec, 1);
        assert_eq!(res, Some(0));

        let res = binary_search(&vec, 2);
        assert_eq!(res, Some(1));

        let res = binary_search(&vec, 3);
        assert_eq!(res, Some(2));

        let res = binary_search(&vec, 4);
        assert_eq!(res, Some(3));
    }

    #[test]
    fn huge_vec() {
        let vec: Vec<i64> = (0..1000000).collect();
        for val in vec![60, 123, 1111, 5455, 132, 112, 145451, 123123, 999999, 0] {
            let ret = binary_search(&vec, val);
            assert_eq!(ret, Some(val as usize));
        }

        let vec: Vec<i64> = (103..10000).collect();
        for val in vec![110, 123, 1111, 5455, 132] {
            let ret = binary_search(&vec, val);
            assert_eq!(ret, Some((val - 103) as usize));
        }
    }
}
