use std::collections::HashMap;

#[test]
fn hash_map_get_disjoint_mut() {
    let mut values = HashMap::from([(0, 1), (1, 2)]);
    assert_eq!(values.get_disjoint_mut([&0, &1]), [Some(&mut 1), Some(&mut 2)]);
}

#[test]
fn hash_map_get_disjoint_mut_missing_key() {
    let mut values = HashMap::from([(0, 1), (1, 2)]);
    assert_eq!(values.get_disjoint_mut([&0, &2]), [Some(&mut 1), None]);
}

#[test]
fn hash_map_get_disjoint_mut_writes() {
    let mut values = HashMap::from([(0, 1), (1, 2)]);
    let [a, b] = values.get_disjoint_mut([&0, &1]);
    *a.unwrap() = 10;
    *b.unwrap() += 20;
    assert_eq!(values, HashMap::from([(0, 10), (1, 22)]));
}

#[test]
fn hash_map_get_disjoint_mut_empty() {
    let mut values: HashMap<i32, i32> = HashMap::new();
    let got: [Option<&mut i32>; 0] = values.get_disjoint_mut([]);
    assert_eq!(got.len(), 0);
}

#[test]
fn hash_map_get_disjoint_mut_borrowed_key() {
    let mut values = HashMap::from([("a".to_string(), 1), ("b".to_string(), 2)]);
    let [a, b] = values.get_disjoint_mut(["a", "b"]);
    *a.unwrap() += 10;
    *b.unwrap() *= 3;
    assert_eq!(values["a"], 11);
    assert_eq!(values["b"], 6);
}

#[test]
fn hash_map_get_disjoint_mut_overlapping_panics() {
    let mut values = HashMap::from([(0, 1)]);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = values.get_disjoint_mut([&0, &0]);
    }));
    let err = r.expect_err("overlapping keys must panic");
    let msg = err.downcast_ref::<&'static str>().copied().or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()));
    assert!(msg.is_some_and(|m| m.contains("duplicate keys")), "{msg:?}");
}

#[test]
fn hash_map_get_disjoint_unchecked_mut() {
    let mut values = HashMap::from([(0, 1), (1, 2)]);
    let got = unsafe { values.get_disjoint_unchecked_mut([&0, &1]) };
    assert_eq!(got, [Some(&mut 1), Some(&mut 2)]);
}
