use std::collections::HashMap;

fn pairings(vec1: Vec<u32>, vec2: Vec<u32>) -> Vec<(u32,u32)> {
    let mut result = Vec::new();
    vec1.iter().for_each(|x| vec2.iter().for_each(|y| result.push((*x,*y))));
    return result;
}

fn frequency_count(vec: Vec<u32>) -> HashMap<u32, u32> {
    let mut hashmap = HashMap::new();
    if vec == [] {
        return hashmap;
    } else {
        vec.iter().for_each(|&x| if hashmap.contains_key(&x) { hashmap.insert(x, hashmap.get(&x).unwrap() + 1); }
            else {
                hashmap.insert(x, 1);
        });
    }
    return hashmap;
}

#[cfg(test)]
mod tests {
    use super::*;
   
// TEST CODE FOR 2 a) and 2b)

    #[test]
    fn pairings_test_1() {
        let vec1 = vec![1,2];
        let vec2 = vec![3,4];
        let result = pairings(vec1, vec2);
        assert_eq!(result, [(1,3),(1,4),(2,3),(2,4)]);
    }
    
    #[test]
    fn pairings_test_2() {
        let vec1 = vec![1];
        let vec2 = vec![1,2,3];
        let result = pairings(vec1, vec2);
        assert_eq!(result, [(1,1),(1,2),(1,3)]);
    }
/*
    #[test]
    fn pairing_test2() {
        let a: u32 = vec![];
        let b = vec![4, 7];
        let c: u32 = vec![];
        let result = pairings(a, b);
        assert_eq!(result, c);
    }
*/
    #[test]
    fn pairings_test_3() {
        let vec1 = vec![];
        let vec2 = vec![1,2,3];
        let result = pairings(vec1, vec2);
        assert_eq!(result, []);
    }

    #[test] 
    fn frequency_count_test_1() {
        let vec = vec![1, 2, 2, 3, 3, 3];
        let result = frequency_count(vec);
        let mut hashmap = HashMap::new();
        hashmap.insert(1,1);
        hashmap.insert(2,2);
        hashmap.insert(3,3);
        assert_eq!(result, hashmap);
    }

    #[test] 
    fn frequency_count_test_2() {
        let vec = vec![3,7,4,7,8,5,7,4,2,8,7,3,8,3,];
        let result = frequency_count(vec);
        let mut hashmap = HashMap::new();
        hashmap.insert(2,1);
        hashmap.insert(3,3);
        hashmap.insert(4,2);
        hashmap.insert(5,1);
        hashmap.insert(7,4);
        hashmap.insert(8,3);
        assert_eq!(result, hashmap);
    }

    #[test] 
    fn frequency_count_test_3() {
        let vec = vec![];
        let result = frequency_count(vec);
        let hashmap = HashMap::new();
        assert_eq!(result,hashmap);
    }




// TEST CODE FOR VECTORS

    #[test]
    fn any_test_vec() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec.iter().any(|&x| x == 2), true);
    }
    
    #[test]
    fn chain_test_vec() {
        let vec1 = vec![1,2,3];
        let vec2 = vec![4,5,6];
        let mut iter = vec1.iter().chain(vec2.iter());
        assert_eq!(iter.any(|&x| x == 6), true);
    }

    #[test]
    fn collect_test_vec() {
        let vec = vec![1,2,3];
        let times3 = vec.iter().map(|&x| x * 3).collect::<Vec<u32>>();
        assert_eq!(times3, vec![3,6,9]);
    }

    #[test]
    fn partition_test_vec() {
        let vec  = vec![1,2,3,5,6,7];
        let (lesser, greater): (Vec<u32>, Vec<u32>) = vec.iter().partition(|&x| x < &4);
        assert_eq!(lesser, vec![1,2,3]);
        assert_eq!(greater, vec![5,6,7]);
    }

    #[test]
    fn try_fold_test_vec() {
        let vec = vec![2,4,6];
        let sum = vec.iter().try_fold(0i8, |temp, &x| temp.checked_add(x));
        assert_eq!(sum, Some(12));
    }

    #[test]
    fn find_map_test_vec() {
        let vec = ["String", "1", "2"];
        let one  = vec.iter().find_map(|x| x.parse().ok());
        assert_eq!(one, Some(1));
    }

    #[test]
    fn rposition_test_vec() {
        let vec = vec![1,2,3];
        let result = vec.iter().rposition(|&x| x == 2);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn max_by_test_vec() {
        let vec = vec![-3, -2, -1, 0 , 1, 2, 3];
        let result = *vec.iter().max_by(|x, y| x.cmp(y)).unwrap();
        assert_eq!(result, 3);
    }
    
    #[test]
    fn unzip_test_vec() {
        let vec = vec![(1,2),(3,4),(5,6)];
        let (odd, even): (Vec<_>, Vec<_>) = vec.iter().cloned().unzip();
        assert_eq!(odd, vec![1,3,5]);
        assert_eq!(even, vec![2,4,6]);
    }

    #[test]
    fn scan_test_vec() {
        let vec = vec![2,3,4];
        let mut iter = vec.iter().scan(2, |s, &x| { *s = *s + x; Some(*s)});
        assert_eq!(iter.next(), Some(4));

    }

    #[test]
    fn flat_map_test_vec() {
        let mut vec = vec!["a b c", "1", "2", "3"];
        let result: String = vec.iter().flat_map(|x| x.split_whitespace()).collect();
        assert_eq!(result, "abc123");
    }

    #[test]
    fn cycle_test_vec() {
        let vec = vec![6,7,8,9,10];
        let mut iter = vec.iter().cycle();
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&7));
    }

// TEST CODE FOR HASHMAPS

    #[test]
    fn any_test_hash() {
        let mut hashmap = HashMap::new();
        hashmap.insert(1,1);
        hashmap.insert(2,2);
        hashmap.insert(3,3);
        assert_eq!(hashmap.iter().any(|(k,v)| *v == 3), true);
    }

    #[test]
    fn chain_test_hash() {
        let mut hashmap1 = HashMap::new();
        let mut hashmap2 = HashMap::new();
        hashmap1.insert(1,1);
        hashmap1.insert(2,2);
        hashmap1.insert(3,3);
        hashmap2.insert(4,4);
        hashmap2.insert(5,5);
        hashmap2.insert(6,6);
        let mut iter = hashmap1.iter().chain(hashmap2.iter());
        assert_eq!(iter.any(|(k,v)| *v == 6), true);
    }

    #[test]
    fn collect_test_hash() {
        let mut hashmap = HashMap::new();
        hashmap.insert(1,1);
        hashmap.insert(2,2);
        hashmap.insert(3,3);
        let mut test = HashMap::new();
        test.insert(1,1);
        test.insert(2,2);
        test.insert(3,3);
        let times3: HashMap<_,_> = test.into_iter().collect();
        assert_eq!(times3, hashmap); 
    }

    #[test]
    fn partition_test_hash() {
        let mut hashmap = HashMap::new();
        hashmap.insert(3,3);
        hashmap.insert(5,5);
        let (lesser, greater): (HashMap<u32,u32>, HashMap<u32,u32>) = hashmap.iter().partition(|(k,v)| *v < &4);
        let mut testlesser = HashMap::new();
        let mut testgreater = HashMap::new();
        testlesser.insert(3,3);
        testgreater.insert(5,5);
        assert_eq!(lesser, testlesser);
        assert_eq!(greater, testgreater);
    }

    #[test]
    fn try_fold_test_hash() {
        let mut hashmap = HashMap::new();
        hashmap.insert(1,2);
        hashmap.insert(2,4);
        hashmap.insert(3,6);
        let sum = hashmap.iter().try_fold(0i8, |temp, (k,v)| temp.checked_add(*v));
        assert_eq!(sum, Some(12));
    }

    #[test]
    fn find_map_test_hash() {
        let mut hashmap = HashMap::new();
        hashmap.insert(1, "String");
        hashmap.insert(2, "1");
       // hashmap.insert(3, "2");
        let one = hashmap.iter().find_map(|(&k,&v)| v.parse().ok());
        assert_eq!(one, Some(1));
    }

    #[test]
    fn rposition_test_hash() {
        let mut hashmap = HashMap::new();
        hashmap.insert(1,1);
        hashmap.insert(2,2);
        hashmap.insert(3,3);
        let (k, v) : (Vec<u32>, Vec<u32>) = hashmap.iter().unzip();
        let i1 = v.iter().rposition(|&x| x == 3).unwrap();
        let i2 = k.iter().rposition(|&x| x == 3).unwrap();
        assert_eq!(i2, i1);
    }   

    #[test]
    fn max_by_test_hash() {
        let mut hm = HashMap::new();
        hm.insert(1, -23);
        hm.insert(2, 25);
        hm.insert(3, -27);
        assert_eq!(hm.iter().max_by(|&x, &y| x.1.cmp(&y.1)).map(|(a, _b)| a), Some(&2));
    }

    #[test]
    fn unzip_test_hash() {
        let mut hashmap = HashMap::new();
        hashmap.insert(10, 2);
        hashmap.insert(3, 1);
        let (k , v) : (Vec<u32>, Vec<u32>) = hashmap.iter().unzip();
        if v == vec![2,1] {
        assert_eq!(v, vec![2,1]);
        } else {
        assert_eq!(v, vec![1,2]);
        } 
    }

    #[test]
    fn scan_test_hash() {
        let mut hashmap = HashMap::new();
        hashmap.insert(1, 2);
        hashmap.insert(2, 2);
        let mut iter = hashmap.iter().scan(2, |temp, (&_i, &x)| {
            *temp = *temp + x;
            Some(*temp)
        });
        assert_eq!(iter.next(), Some(4));
    }
    
    #[test]
    fn flat_map_test_hash() {
        let mut hashmap = HashMap::new();
        hashmap.insert(1, "123");
        hashmap.insert(2, "123");
        let result: String = hashmap.iter().flat_map(|(&k, &v)| v.split_whitespace()).collect();

        assert_eq!(result, "123123");
    }

    #[test]
    fn cycle_test_hash() {
        let mut hashmap = HashMap::new();
        hashmap.insert(1, 1);
        hashmap.insert(2, 2);
        hashmap.insert(3, 3);
        let mut iter = hashmap.iter().cycle();
        assert_eq!(iter.any(|(_x, y)| y > &1), true);
    }

}
