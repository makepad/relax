use std::collections::{HashMap, HashSet};

#[test]
fn hash_set_try_reserve() {
    let mut s: HashSet<i32> = HashSet::new();
    s.try_reserve(10).unwrap();
    assert!(s.capacity() >= 10);
}

#[test]
fn hash_set_extract_if() {
    let mut s = HashSet::from([1, 2, 3, 4, 5, 6]);
    let removed: HashSet<i32> = s.extract_if(|x| x % 2 == 0).collect();
    assert_eq!(removed, HashSet::from([2, 4, 6]));
    assert_eq!(s, HashSet::from([1, 3, 5]));
}

#[test]
fn hash_set_extract_if_partial_keeps_rest() {
    let mut s: HashSet<i32> = (0..10).collect();
    {
        let mut it = s.extract_if(|x| *x < 5);
        let _ = it.next();
    }
    // one next() removes at most one element (the first `x < 5` it finds); the rest stays
    assert_eq!(s.len(), 9);
    for x in 5..10 {
        assert!(s.contains(&x));
    }
}

#[test]
fn hash_set_extract_if_pred_calls() {
    let mut s: HashSet<i32> = (0..8).collect();
    let mut calls = 0;
    s.extract_if(|_| {
        calls += 1;
        false
    })
    .for_each(drop);
    assert_eq!(calls, 8);
    assert_eq!(s.len(), 8);
}

#[test]
fn hash_map_set_unwind_safe() {
    fn assert<T: std::panic::UnwindSafe + std::panic::RefUnwindSafe>() {}
    assert::<HashMap<i32, i32>>();
    assert::<HashSet<i32>>();
}
